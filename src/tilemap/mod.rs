use std::cmp::Ordering;
use std::collections::VecDeque;

use bevy::{prelude::*, utils::HashMap};

pub use self::chunk::Chunk;
pub use self::tile::Tile;

pub mod chunk;
pub mod tile;

// Number of pixels per tile
pub const TILE_SIZE: usize = 8;

// Number of tiles per chunk
pub const CHUNK_SIZE: usize = 16;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        // Add the systems to both post update and post startup to help with the initialization process.
        app.add_systems(
            PostUpdate,
            (handle_tilemap_tasks, chunk_texture_update).chain(),
        )
        .add_systems(
            PostStartup,
            (handle_tilemap_tasks, chunk_texture_update).chain(),
        );
    }
}

#[derive(Bundle, Default)]
pub struct TilemapBundle {
    pub tilemap: Tilemap,
    pub visibility: VisibilityBundle,
    pub transform: TransformBundle,
}

#[derive(PartialEq)]
pub enum TilemapTask {
    MakeChunk((i32, i32)),
    SetTile {
        loc: (i32, i32),
        entity: Option<Entity>,
    },
}

#[derive(Default, Component)]
pub struct Tilemap {
    pub chunks: HashMap<(i32, i32), Entity>,
    tasks: VecDeque<TilemapTask>,
}

impl Tilemap {
    pub fn get_tile(
        &mut self,
        loc: (i32, i32),
        chunks: &mut Query<&mut Chunk>,
        generate_if_missing: bool,
    ) -> Option<Entity> {
        let chunk_loc = (loc.0 / CHUNK_SIZE as i32, loc.1 / CHUNK_SIZE as i32);
        if let Some(chunk_entity) = self.chunks.get(&chunk_loc) {
            let mut chunk = chunks
                .get_mut(*chunk_entity)
                .expect("Chunk Entity Should Exist");
            let tile_loc = (loc.0 % CHUNK_SIZE as i32, loc.1 % CHUNK_SIZE as i32);
            return chunk
                .get_tile(tile_loc)
                .expect("Chunk location should be in range");
        } else if generate_if_missing {
            // The Chunk is missing, so generate the chunk.
            self.tasks.push_back(TilemapTask::MakeChunk(chunk_loc));
        }

        None
    }

    pub fn set_tile(
        &mut self,
        loc: (i32, i32),
        mut tile: Tile,
        additional_components: impl Bundle,
        commands: &mut Commands,
    ) {
        // Ensure the chunk exists
        self.require_chunk(chunk_from_location(loc));

        // Init the tile's location
        tile.location = Some(loc);

        // Spawn the tile
        let entity = commands.spawn((tile, additional_components)).id();

        // Prepare the task
        self.tasks.push_back(TilemapTask::SetTile {
            loc,
            entity: Some(entity),
        });
    }

    pub fn delete_tile(&mut self, loc: (i32, i32)) {
        if !self.has_chunk(chunk_from_location(loc)) {
            // No need to delete something that isn't there.
            return;
        }
        // Ensure the chunk exists
        self.require_chunk(chunk_from_location(loc));

        // Prepare the task
        self.tasks
            .push_back(TilemapTask::SetTile { loc, entity: None });
    }

    pub fn require_chunk(&mut self, chunk_loc: (i32, i32)) {
        if !self.chunks.contains_key(&chunk_loc) {
            self.tasks.push_back(TilemapTask::MakeChunk(chunk_loc));
        }
    }

    pub fn has_chunk(&mut self, chunk_loc: (i32, i32)) -> bool {
        self.chunks.contains_key(&chunk_loc)
    }
}

pub fn handle_tilemap_tasks(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut tilemaps: Query<(Entity, &mut Tilemap)>,
    mut chunks: Query<&mut Chunk>,
) {
    for (entity, mut tilemap) in &mut tilemaps {
        let mut re_add_tasks = VecDeque::new();
        while let Some(task) = tilemap.tasks.pop_front() {
            match task {
                TilemapTask::MakeChunk(loc) => {
                    // Ensure the chunk task isn't there already
                    if !tilemap.chunks.contains_key(&loc) {
                        let mut chunk_entity = commands.spawn_empty();
                        chunk_entity.set_parent_in_place(entity);

                        // Create the chunk
                        let chunk = Chunk::new(loc, &mut images);

                        // Insert the chunk's components
                        chunk_entity.insert((
                            SpriteBundle {
                                texture: chunk.image_handle.clone_weak(),
                                transform: Transform::from_xyz(
                                    loc.0 as f32 * CHUNK_SIZE as f32,
                                    loc.1 as f32 * CHUNK_SIZE as f32,
                                    0.0,
                                )
                                .with_scale(Vec3::splat(1.0 / TILE_SIZE as f32)),
                                ..default()
                            },
                            chunk,
                        ));

                        // Add the chunk to the tilemap
                        tilemap.chunks.insert(loc, chunk_entity.id());
                    }
                }
                TilemapTask::SetTile { loc, entity } => {
                    let chunk_loc = chunk_from_location(loc);

                    let chunk_entity = *tilemap.chunks.get(&chunk_loc).expect("Chunk should exist");

                    if let Ok(mut chunk) = chunks.get_mut(chunk_entity) {
                        let tile_loc = tile_from_location(loc);
                        let tile_index = tile_loc.0 + tile_loc.1 * CHUNK_SIZE;

                        // Ensure the old entity is despawned

                        // TODO: Add a component to the system that signifies the tile is being despawned,
                        // then despawn at the beginning of the next frame, in the PreUpdate stage.
                        if let Some(tile_entity) = chunk.tiles[tile_index] {
                            commands.entity(tile_entity).despawn_recursive();
                        }

                        // Put the tile under the chunk entity.
                        if let Some(entity) = entity {
                            commands.entity(entity).set_parent_in_place(chunk_entity);
                        }

                        chunk.tiles[tile_index] = entity;

                        chunk.request_update();
                    } else {
                        // The chunk hasn't spawned yet, so re-add the tile creation task, so it can run the next frame
                        re_add_tasks.push_front(TilemapTask::SetTile { loc, entity })
                    }
                }
            }
        }
        tilemap.tasks.append(&mut re_add_tasks);
    }
}

pub fn chunk_from_location(loc: (i32, i32)) -> (i32, i32) {
    (align_loc_to_chunk(loc.0), align_loc_to_chunk(loc.1))
}

pub fn align_loc_to_chunk(mut loc: i32) -> i32 {
    match loc.cmp(&0) {
        Ordering::Greater => loc / CHUNK_SIZE as i32,
        Ordering::Less => {
            let mut result = 0;
            while loc < 0 {
                loc += CHUNK_SIZE as i32;
                result -= 1;
            }
            result
        }
        Ordering::Equal => 0,
    }
}

pub fn tile_from_location(loc: (i32, i32)) -> (usize, usize) {
    let loc_x = loc.0.rem_euclid(CHUNK_SIZE as i32) as usize;
    let loc_y = loc.1.rem_euclid(CHUNK_SIZE as i32) as usize;

    (loc_x, loc_y)
}

// Updates all textures for the tiles if they are marked as dirty.
pub fn chunk_texture_update(
    mut chunks: Query<&mut Chunk>,
    mut tiles: Query<&mut Tile>,
    mut images: ResMut<Assets<Image>>,
) {
    for mut chunk in &mut chunks {
        if chunk.dirty {
            chunk.update_texture(&mut images, &mut tiles);
        }
    }
}
