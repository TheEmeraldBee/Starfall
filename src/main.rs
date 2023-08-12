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
        // Add the resources
        .add_systems(Startup, start_system)
        // Add the test system
        .add_systems(Update, test_system)
        // Run the Game
        .run();
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

    for x in 0..10 {
        for y in 0..10 {
            let chunk = tilemap.get_chunk((x, y), &mut images, &mut commands);

            chunk
                .set_tile(
                    (5, 5),
                    Some(Tile::new_fill(Color::WHITE)),
                    (),
                    &mut commands,
                )
                .unwrap();

            chunk.request_update();
        }
    }
}

fn test_system(
    mut tiles: ResMut<Tilemap>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut chunks: Query<&mut ChunkComponent>,
) {
    for chunk in &mut chunks {
        let chunk = tiles.get_chunk(chunk.location, &mut images, &mut commands);

        chunk
            .set_tile((0, 0), Some(Tile::new_fill(Color::BLUE)), (), &mut commands)
            .unwrap();

        chunk.request_update();
    }
}
