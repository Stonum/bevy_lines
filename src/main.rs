// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod resources;
use resources::*;

mod components;

mod systems;
use systems::*;

mod events;
use events::*;

mod next_balls;
use next_balls::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Playing,
    Menu,
}

fn main() {
    let mut app = App::new();
    app.add_state::<GameState>().add_plugins(
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

    app.insert_resource(ClearColor(Color::BLACK))
        .init_resource::<BoardAssets>()
        .init_resource::<Board>()
        .init_resource::<Game>()
        .init_resource::<BallAssets>();

    app.add_plugin(NextBallsPlugin);

    app.add_startup_system(spawn_camera)
        .add_startup_system(spawn_board)
        .add_startup_system(spawn_score_fields)
        .add_event::<SpawnBallsEvent>()
        .add_event::<SelectBallEvent>()
        .add_event::<MoveBallEvent>()
        .add_system(render_score_text)
        .add_system(render_balls)
        .add_system(animate_ball_system)
        .add_systems((handle_mouse_clicks, spawn_new_balls).chain())
        .run();
}

fn spawn_camera(mut commands: Commands) {
    // Create a camera
    commands.spawn(Camera2dBundle::default());
}
