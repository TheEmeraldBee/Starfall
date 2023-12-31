use bevy::{prelude::*, render::render_resource::Extent3d};

use crate::tilemap::{CHUNK_SIZE, TILE_SIZE};

use super::tile::Tile;

#[derive(Component, Debug, Clone)]
pub struct Chunk {
    pub location: (i32, i32),
    pub tiles: Vec<Option<Entity>>,
    pub image: Image,
    pub image_handle: Handle<Image>,
    pub dirty: bool,
}

impl Chunk {
    pub fn new(location: (i32, i32), images: &mut ResMut<Assets<Image>>) -> Self {
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
            dirty: true,
        }
    }

    pub fn get_tile(&mut self, location: (i32, i32)) -> anyhow::Result<Option<Entity>> {
        if location.0 >= CHUNK_SIZE as i32
            || location.1 >= CHUNK_SIZE as i32
            || location.0 < 0
            || location.1 < 0
        {
            return Err(anyhow!(
                "Expected tile between 0, 0 and {0}, {0} but got {1}, {2}",
                CHUNK_SIZE - 1,
                location.0,
                location.1
            ));
        }

        Ok(self.tiles[location.0 as usize + location.1 as usize * CHUNK_SIZE])
    }

    /// Updates the texture on the tile.
    /// WARNING: Extremely ineffective if used often.
    /// Prefer to use request_update() unless absolutely necissary
    pub fn update_texture(
        &mut self,
        images: &mut ResMut<Assets<Image>>,
        tiles: &mut Query<&mut Tile>,
    ) {
        self.dirty = false;
        let mut data = vec![
            vec![Color::rgba(0.0, 0.0, 0.0, 1.0); CHUNK_SIZE * TILE_SIZE];
            CHUNK_SIZE * TILE_SIZE
        ];
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let tile = self.get_tile((x as i32, y as i32)).unwrap();

                if let Some(tile) = tile {
                    for pixel_x in 0..TILE_SIZE {
                        for pixel_y in 0..TILE_SIZE {
                            // Inverse of y * pixels per tile + the current pixel
                            let pixel_index_y = ((CHUNK_SIZE - 1 - y) * TILE_SIZE) + pixel_y;
                            // x * pixels per tile + the current pixel
                            let pixel_index_x = (x * TILE_SIZE) + pixel_x;

                            data[pixel_index_y][pixel_index_x] = tiles
                                .get_mut(tile)
                                .expect("Tile Entity Should Exist")
                                .get_pixel((pixel_x, pixel_y))
                                .unwrap();
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
    /// Renders after the update stage.
    pub fn request_update(&mut self) {
        self.dirty = true;
    }
}
