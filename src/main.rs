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
        // Run the Game
        .run();
}

fn start_system(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.viewport_origin = Vec2::new(0.5, 0.5);
    camera_bundle.projection.scaling_mode = ScalingMode::WindowSize(25.6);
    camera_bundle.projection.scale = -2.0;
    commands.spawn(camera_bundle);

    let mut tilemap = Tilemap::default();

    for x in -128..128 {
        for y in -128..128 {
            tilemap.set_tile((x, y), Tile::new_fill(Color::WHITE), (), &mut commands);
        }
    }

    // Spawn the tilemap
    commands.spawn(TilemapBundle {
        tilemap,
        ..default()
    });
}
