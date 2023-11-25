// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board_plugin;
mod events;
mod game_score_plugin;
mod leader_board_plugin;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use board_plugin::BoardPlugin;
use events::*;
use game_score_plugin::GameScorePlugin;
use leader_board_plugin::LeaderBoardPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

pub struct GameOptions;
impl GameOptions {
    pub const TILE_SIZE: f32 = 45.0;
    pub const TILE_PADDING: f32 = 5.0;
    pub const TILE_COUNT: u8 = 9;
    pub const BOARD_SIZE: f32 = GameOptions::TILE_SIZE * GameOptions::TILE_COUNT as f32;
    pub const BALL_SIZE: f32 = 35.0;
    pub const MIN_BALLS_ON_LINE: usize = 5;
    pub const WINDOW_WIDTH: f32 = 600.;
    pub const WINDOW_HEIGHT: f32 = 800.;
}

fn main() {
    let mut app = App::new();
    app.add_state::<GameState>().add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Lines".into(),
                    resolution: (GameOptions::WINDOW_HEIGHT, GameOptions::WINDOW_WIDTH).into(),
                    ..default()
                }),
                ..default()
            })
            .build()
            .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
    );

    app.insert_resource(ClearColor(Color::BLACK));
    app.add_event::<IncrementCurrentGameScore>();

    app.add_plugins((BoardPlugin, LeaderBoardPlugin, GameScorePlugin));

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
