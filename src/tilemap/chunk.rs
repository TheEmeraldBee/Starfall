use bevy::{prelude::*, render::render_resource::Extent3d};

use crate::tilemap::{CHUNK_SIZE, TILE_SIZE};

use super::tile::{Tile, TileComponent};

#[derive(Component)]
pub struct ChunkComponent {
    pub location: (i32, i32),
}

pub struct Chunk {
    pub location: (i32, i32),
    pub tiles: Vec<Option<Tile>>,
    pub image: Image,
    pub image_handle: Handle<Image>,
    pub dirty: bool,
    pub entity: Entity,
}

impl Chunk {
    pub fn new(location: (i32, i32), images: &mut ResMut<Assets<Image>>, entity: Entity) -> Self {
        let mut data = vec![];
        for _ in 0..((CHUNK_SIZE * CHUNK_SIZE) * TILE_SIZE * TILE_SIZE) {
            data.append(&mut Color::rgba(0.0, 0.0, 0.0, 0.0).as_rgba_u8().to_vec());
        }

        assert_eq!(
            data.len(),
            ((CHUNK_SIZE * CHUNK_SIZE) * TILE_SIZE * TILE_SIZE) * 4
        );

        let image = Image::new(
            Extent3d {
                width: CHUNK_SIZE as u32 * TILE_SIZE as u32,
                height: CHUNK_SIZE as u32 * TILE_SIZE as u32,
                ..default()
            },
            bevy::render::render_resource::TextureDimension::D2,
            data,
            bevy::render::render_resource::TextureFormat::Rgba8Unorm,
        );

        let tiles = vec![None; CHUNK_SIZE * CHUNK_SIZE];

        Self {
            location,
            tiles,
            image: image.clone(),
            image_handle: images.add(image),
            dirty: false,
            entity,
        }
    }

    pub fn get_tile(&mut self, location: (usize, usize)) -> anyhow::Result<Option<&mut Tile>> {
        if location.0 >= CHUNK_SIZE || location.1 >= CHUNK_SIZE {
            return Err(anyhow!(
                "Expected tile between 0, 0 and {0}, {0} but got {1}, {2}",
                CHUNK_SIZE - 1,
                location.0,
                location.1
            ));
        }

        Ok(self.tiles[location.0 + location.1 * CHUNK_SIZE].as_mut())
    }

    pub fn set_tile(
        &mut self,
        location: (usize, usize),
        tile: Option<Tile>,
        additional_components: impl Bundle,
        commands: &mut Commands,
    ) -> anyhow::Result<()> {
        if location.0 >= CHUNK_SIZE || location.1 >= CHUNK_SIZE {
            return Err(anyhow!(
                "Expected tile between 0, 0 and {0}, {0} but got {1}, {2}",
                CHUNK_SIZE - 1,
                location.0,
                location.1
            ));
        }

        let cur_tile = &mut self.tiles[location.0 + location.1 * CHUNK_SIZE];

        if let Some(cur_tile) = cur_tile {
            commands
                .entity(cur_tile.entity.expect("Tile Should be fully initialized"))
                .despawn_recursive();
        }

        if let Some(mut tile) = tile {
            tile.entity = Some(
                commands
                    .spawn((
                        TileComponent {
                            chunk_loc: self.location,
                            loc: location,
                        },
                        additional_components,
                    ))
                    .id(),
            );
            *cur_tile = Some(tile);
        } else {
            *cur_tile = None;
        }

        Ok(())
    }

    /// Updates the texture on the tile.
    /// WARNING: Extremely ineffective if used often.
    /// Prefer to use request_update() unless absolutely necissary
    pub fn update_texture(&mut self, images: &mut ResMut<Assets<Image>>) {
        self.dirty = false;
        let mut data = vec![vec![Color::BLACK; CHUNK_SIZE * TILE_SIZE]; CHUNK_SIZE * TILE_SIZE];
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let tile = self.get_tile((x, y)).unwrap();
                if let Some(tile) = tile {
                    for pixel_x in 0..TILE_SIZE {
                        for pixel_y in 0..TILE_SIZE {
                            data[x * TILE_SIZE + pixel_x][y * TILE_SIZE + pixel_y] =
                                tile.get_pixel((pixel_x, pixel_y)).unwrap();
                        }
                    }
                }
            }
        }

        let data: Vec<_> = data
            .into_iter()
            .flatten()
            .flat_map(|color| color.as_rgba_u8())
            .collect();

        self.image.data = data;

        self.image_handle = images.set(self.image_handle.clone(), self.image.clone());
    }

    /// Requests a mesh and render update from the system.
    pub fn request_update(&mut self) {
        self.dirty = true;
    }
}
