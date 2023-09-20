// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board_plugin;
mod events;
mod game_score_plugin;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use board_plugin::BoardPlugin;
use events::*;
use game_score_plugin::GameScorePlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
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

    app.insert_resource(ClearColor(Color::BLACK));
    app.add_event::<IncrementCurrentGameScore>();

    app.add_plugins((BoardPlugin, GameScorePlugin));

    app.add_systems(Startup, spawn_camera)
        .add_systems(Update, handle_keyboard)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    // Create a camera
    commands.spawn(Camera2dBundle::default());
}

pub fn handle_keyboard(keys: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keys.pressed(KeyCode::X) {
        game_state.set(GameState::GameOver);
    }
    if keys.pressed(KeyCode::P) {
        game_state.set(GameState::Playing);
    }
}
