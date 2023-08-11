use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::camera::ScalingMode,
    window::{WindowMode, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_turborand::{prelude::RngPlugin, DelegatedRng, RngComponent};
use tile::{component::Tile, TILE_SIZE};

#[macro_use]
extern crate anyhow;

pub mod tile;

pub const CHUNK_SIZE: usize = 32;

#[derive(Component, Clone, Debug)]
pub struct Chunk {
    pub location: (i32, i32),
    pub tiles: [Option<Entity>; CHUNK_SIZE * CHUNK_SIZE],
}

impl Chunk {
    pub fn new(location: (i32, i32)) -> Self {
        Self {
            location,
            tiles: [None; CHUNK_SIZE * CHUNK_SIZE],
        }
    }

    pub fn set_tile(
        &mut self,
        loc: (u32, u32),
        tile: Tile,
        additional_components: impl Bundle,
        commands: &mut Commands,
    ) -> anyhow::Result<()> {
        if loc.0 > CHUNK_SIZE as u32 || loc.1 > CHUNK_SIZE as u32 {
            return Err(anyhow!(
                "loc should be in range between 0, 0 and {0}, {0}",
                CHUNK_SIZE - 1
            ));
        }

        let chunk_loc_x = self.location.0 as f32 * CHUNK_SIZE as f32;
        let chunk_loc_y = self.location.1 as f32 * CHUNK_SIZE as f32;
        let tile_id = commands
            .spawn((
                SpriteBundle {
                    texture: tile.get_image_handle(),
                    transform: Transform::from_xyz(
                        chunk_loc_x + loc.0 as f32,
                        chunk_loc_y + loc.1 as f32,
                        0.0,
                    )
                    .with_scale(Vec3::splat(1.0 / TILE_SIZE as f32)),
                    ..Default::default()
                },
                tile,
                additional_components,
            ))
            .id();

        self.tiles[loc.0 as usize + loc.1 as usize * CHUNK_SIZE] = Some(tile_id);

        Ok(())
    }

    pub fn get_tile(&mut self, loc: (u32, u32)) -> anyhow::Result<Option<Entity>> {
        if loc.0 > CHUNK_SIZE as u32 || loc.1 > CHUNK_SIZE as u32 {
            return Err(anyhow!(
                "loc should be in range between 0, 0 and {0}, {0}",
                CHUNK_SIZE - 1
            ));
        }
        Ok(self.tiles[loc.0 as usize + loc.1 as usize * CHUNK_SIZE])
    }

    pub fn remove_tile(&mut self, loc: (u32, u32)) -> anyhow::Result<()> {
        if loc.0 > CHUNK_SIZE as u32 || loc.1 > CHUNK_SIZE as u32 {
            return Err(anyhow!(
                "loc should be in range between 0, 0 and {0}, {0}",
                CHUNK_SIZE - 1
            ));
        }
        self.tiles[loc.0 as usize + loc.1 as usize * CHUNK_SIZE] = None;
        Ok(())
    }
}

#[derive(Component)]
pub struct TimerComponent(Timer);

fn main() {
    App::new()
        // Add Default Plugins with custom rendering and window settings
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Starlight".to_string(),
                        mode: WindowMode::Windowed,
                        resolution: WindowResolution::new(1920.0, 1080.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        // Add the inspector and the rng system to the game
        .add_plugins((WorldInspectorPlugin::new(), RngPlugin::new()))
        // Add Debug Plugins
        .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin))
        // Add the systems
        .add_systems(Startup, start_system)
        .add_systems(Update, test_system)
        // Run the Game
        .run();
}

fn start_system(mut commands: Commands, mut textures: ResMut<Assets<Image>>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.viewport_origin = Vec2::new(0.0, 0.0);
    camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(25.6);
    commands.spawn(camera_bundle);

    // Create a blank tile.
    let tile = Tile::new_solid(Color::BLACK, &mut textures);

    for x in 0..3 {
        for y in 0..2 {
            let mut chunk = Chunk::new((x, y));

            for x in 0..32 {
                for y in 0..32 {
                    chunk
                        .set_tile(
                            (x, y),
                            tile.clone(),
                            (
                                // RngComponent::new(),
                                // TimerComponent(Timer::from_seconds(0.25, TimerMode::Repeating)),
                            ),
                            &mut commands,
                        )
                        .expect("x, y should be in range 0 and 32");
                }
            }
        }
    }
}

fn test_system(
    time: Res<Time>,
    mut textures: ResMut<Assets<Image>>,
    mut query: Query<(&mut Tile, &mut RngComponent, &mut TimerComponent)>,
) {
    for (mut tile, mut rng, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let x = rng.usize(0..8);
            let y = rng.usize(0..8);

            tile.set_pixel((x, y), Color::rgba(1.0, 1.0, 1.0, 0.2))
                .expect("Pixel should be in a valid location");
            tile.update_image(&mut textures);
        }
    }
}
