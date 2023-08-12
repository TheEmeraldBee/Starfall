use bevy::{prelude::*, utils::HashMap};

pub use self::chunk::{Chunk, ChunkComponent};
pub use self::tile::{Tile, TileComponent};

pub mod chunk;
pub mod tile;

// Number of pixels per tile
pub const TILE_SIZE: usize = 8;

// Number of tiles per chunk
pub const CHUNK_SIZE: usize = 16;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Tilemap::default())
            .add_systems(PostUpdate, chunk_texture_update);
    }
}

#[derive(Default, Resource)]
pub struct Tilemap {
    pub chunks: HashMap<(i32, i32), Chunk>,
}

impl Tilemap {
    pub fn get_chunk(
        &mut self,
        loc: (i32, i32),
        images: &mut ResMut<Assets<Image>>,
        commands: &mut Commands,
    ) -> &mut Chunk {
        if self.chunks.contains_key(&loc) {
            self.chunks.get_mut(&loc).expect("Chunk should exist")
        } else {
            // Chunk Doesn't exist, create it.
            let mut entity = commands.spawn_empty();

            let chunk = Chunk::new(loc, images, entity.id());

            entity.insert((
                SpriteBundle {
                    texture: chunk.image_handle.clone(),
                    transform: Transform::from_xyz(
                        loc.0 as f32 * CHUNK_SIZE as f32,
                        loc.1 as f32 * CHUNK_SIZE as f32,
                        0.0,
                    )
                    .with_scale(Vec3::splat(1.0 / TILE_SIZE as f32)),
                    ..default()
                },
                ChunkComponent { location: loc },
            ));

            self.chunks.insert(loc, chunk);
            self.chunks.get_mut(&loc).expect("Chunk should exist")
        }
    }
}

// Updates all textures for the tiles if they are marked as dirty.
pub fn chunk_texture_update(mut tilemap: ResMut<Tilemap>, mut images: ResMut<Assets<Image>>) {
    for chunk in &mut tilemap.chunks {
        if chunk.1.dirty {
            chunk.1.update_texture(&mut images);
        }
    }
}
