// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod plugins;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_simple_text_input::TextInputPlugin;

use plugins::board::BoardPlugin;
use plugins::game_score::GameScorePlugin;
use plugins::layout::LayoutPlugin;
use plugins::leader_board::LeaderBoardPlugin;
use plugins::menu::MenuPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
    Restarting,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum LeaderBoardState {
    #[default]
    Hide,
    Show,
    InputName,
}

pub struct GameOptions;
impl GameOptions {
    pub const TILE_SIZE: f32 = 45.0;
    pub const TILE_PADDING: f32 = 2.5;
    pub const TILE_COUNT: u8 = 9;
    pub const BOARD_SIZE: f32 = GameOptions::TILE_SIZE * GameOptions::TILE_COUNT as f32;
    pub const BALL_SIZE: f32 = 35.0;
    pub const MIN_BALLS_ON_LINE: usize = 5;
    pub const WINDOW_WIDTH: f32 = 900.;
    pub const WINDOW_HEIGHT: f32 = 600.;

    pub const BOARD_COLOR: Color = Color::rgb(0.53, 0.53, 0.53);
    pub const TILE_COLOR: Color = Color::rgb(0.88, 0.88, 0.88);
}

fn main() {
    let mut app = App::new();
    app.add_state::<GameState>()
        .add_state::<LeaderBoardState>()
        .add_plugins(
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
        )
        .add_plugins(TextInputPlugin);

    app.insert_resource(ClearColor(Color::BLACK));

    app.add_plugins((
        LayoutPlugin,
        BoardPlugin,
        LeaderBoardPlugin,
        GameScorePlugin,
        MenuPlugin,
    ));

    app.add_systems(Startup, spawn_camera).run();
}

fn spawn_camera(mut commands: Commands) {
    // Create a camera
    commands.spawn(Camera2dBundle::default());
}
