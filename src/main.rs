// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod resources;
use resources::*;

mod components;
use components::*;

mod systems;
use systems::*;

mod events;
use events::*;

const TILE_SIZE: f32 = 45.0;
const TILE_PADDING: f32 = 5.0;
const TILE_COUNT: u8 = 9;
const BALL_SIZE: f32 = 35.0;
const MIN_BALLS_INLINE: usize = 5;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const BOARD_COLOR: Color = Color::rgb(0.53, 0.53, 0.53);
const TILE_COLOR: Color = Color::rgb(0.88, 0.88, 0.88);

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Lines".into(),
                    resolution: (800., 600.).into(),
                    ..default()
                }),
                ..default()
            })
            .build()
            .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
    );

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_type::<Coordinates>()
        .register_type::<BallColor>()
        .register_type::<Ball>();

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Board::new(TILE_COUNT, TILE_SIZE))
        .init_resource::<BallAssets>();

    app.add_startup_system(spawn_camera)
        .add_startup_system(spawn_board)
        .add_startup_system(spawn_next_board)
        .add_event::<SpawnBallsEvent>()
        .add_event::<SelectBallEvent>()
        .add_event::<MoveBallEvent>()
        .add_event::<ChangeNextBalls>()
        .add_system(render_next_balls)
        .add_system(render_balls)
        .add_system(animate_ball_system)
        .add_systems((handle_mouse_clicks, spawn_new_balls, change_next_color).chain())
        .run();
}

fn spawn_camera(mut commands: Commands) {
    // Create a camera
    commands.spawn(Camera2dBundle::default());
}
