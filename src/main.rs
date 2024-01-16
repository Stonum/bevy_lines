// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board_plugin;
mod events;
mod game_score_plugin;
mod leader_board_plugin;
mod menu_plugin;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use board_plugin::BoardPlugin;
use events::*;
use game_score_plugin::GameScorePlugin;
use leader_board_plugin::LeaderBoardPlugin;
use menu_plugin::MenuPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
    Restarting,
    Leaderboard,
}

pub struct GameOptions;
impl GameOptions {
    pub const TILE_SIZE: f32 = 45.0;
    pub const TILE_PADDING: f32 = 5.0;
    pub const TILE_COUNT: u8 = 9;
    pub const BOARD_SIZE: f32 = GameOptions::TILE_SIZE * GameOptions::TILE_COUNT as f32;
    pub const BALL_SIZE: f32 = 35.0;
    pub const MIN_BALLS_ON_LINE: usize = 5;
    pub const WINDOW_WIDTH: f32 = 800.;
    pub const WINDOW_HEIGHT: f32 = 600.;
}

fn main() {
    let mut app = App::new();
    app.add_state::<GameState>().add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Lines".into(),
                    resolution: (GameOptions::WINDOW_WIDTH, GameOptions::WINDOW_HEIGHT).into(),
                    // Bind to canvas included in `index.html`
                    canvas: Some("#bevy".to_owned()),
                    // The canvas size is constrained in index.html and build/web/styles.css
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5 and Ctrl+R
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .build()
            .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
    );

    app.insert_resource(ClearColor(Color::BLACK));
    app.add_event::<IncrementCurrentGameScore>();

    app.add_plugins((BoardPlugin, LeaderBoardPlugin, GameScorePlugin, MenuPlugin));

    app.add_systems(Startup, spawn_camera).run();
}

fn spawn_camera(mut commands: Commands) {
    // Create a camera
    commands.spawn(Camera2dBundle::default());
}
