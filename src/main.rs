use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::camera::ScalingMode,
    window::{WindowMode, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_turborand::prelude::RngPlugin;
use tilemap::*;

pub mod tilemap;

#[macro_use]
extern crate anyhow;

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
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, start_system)
        .add_systems(Update, test_system)
        .insert_resource(TestResource {
            timer: Timer::from_seconds(0.001, TimerMode::Repeating),
            x: -10 * CHUNK_SIZE as i32,
            y: -10 * CHUNK_SIZE as i32,
        })
        // Run the Game
        .run();
}

#[derive(Resource)]
pub struct TestResource {
    timer: Timer,
    x: i32,
    y: i32,
}

fn start_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut tilemap: ResMut<Tilemap>,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.viewport_origin = Vec2::new(0.5, 0.5);
    camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(25.6);
    commands.spawn(camera_bundle);

    for x in -10..10 {
        for y in -10..10 {
            let chunk = tilemap.get_chunk((x, y), &mut images, &mut commands);
            chunk.request_update();
        }
    }
}

fn test_system(
    mut tiles: ResMut<Tilemap>,
    mut resource: ResMut<TestResource>,
    mut commands: Commands,
    time: Res<Time>,
) {
    resource.timer.tick(time.delta());
    for _ in 0..resource.timer.times_finished_this_tick() {
        let chunk_loc = tiles.get_chunk_loc((resource.x, resource.y));
        if let Some(chunk) = tiles.try_get_chunk(chunk_loc) {
            chunk.request_update();
            chunk
                .set_tile(
                    (
                        resource.x.unsigned_abs() as usize % CHUNK_SIZE,
                        resource.y.unsigned_abs() as usize % CHUNK_SIZE,
                    ),
                    Some(Tile::new_fill(Color::WHITE)),
                    (),
                    &mut commands,
                )
                .unwrap();
        }

        resource.x += 1;
        if resource.x >= 48 {
            resource.x = 0;
            resource.y += 1;
        }
    }
}
