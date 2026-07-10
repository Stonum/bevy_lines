// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod plugins;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_simple_text_input::TextInputPlugin;

#[cfg(target_arch = "wasm32")]
use bevy::input::InputSystem;
#[cfg(target_arch = "wasm32")]
use bevy::ui::UiSystem;
#[cfg(target_arch = "wasm32")]
use bevy::window::{CursorMoved, PrimaryWindow};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

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
    pub const TILE_PADDING: f32 = 2.0;
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
                        resolution: window_resolution(),
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        // Keep a fixed internal resolution; the canvas is scaled to fit
                        // the viewport purely via CSS (see build/web/styles.css), so
                        // browser DPI/zoom never affects UI layout pixel math.
                        fit_canvas_to_parent: false,
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

    #[cfg(target_arch = "wasm32")]
    app.add_systems(
        PreUpdate,
        correct_cursor_position_for_canvas_scale
            .after(InputSystem)
            .before(UiSystem::Focus),
    );

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

/// Winit (web backend) reports cursor position as `offset * devicePixelRatio`, and
/// converts that back to logical coordinates using Bevy's own `scale_factor()` --
/// which we've pinned to 1.0 via `scale_factor_override`. It also has no idea that
/// the canvas is visually scaled by CSS independently of the resolution it manages.
/// Both effects need undoing to map a real click back into the fixed 900x600 game
/// space that all our UI layout math assumes.
#[cfg(target_arch = "wasm32")]
fn correct_cursor_position_for_canvas_scale(
    mut evr_cursor: EventReader<CursorMoved>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    // Only recompute on a genuine new event from winit; otherwise
    // `physical_cursor_position` already holds our own previous correction, and
    // re-applying the formula to it would double-correct.
    if evr_cursor.iter().last().is_none() {
        return;
    }

    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };
    let Some(raw_physical) = window.physical_cursor_position() else {
        return;
    };

    let Some(browser) = web_sys::window() else {
        return;
    };
    let Some(canvas) = browser
        .document()
        .and_then(|doc| doc.get_element_by_id("bevy"))
        .and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
    else {
        return;
    };

    let rect = canvas.get_bounding_client_rect();
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return;
    }

    let dpr = browser.device_pixel_ratio();
    let offset_x = raw_physical.x as f64 / dpr;
    let offset_y = raw_physical.y as f64 / dpr;

    let corrected = Vec2::new(
        (offset_x * (GameOptions::WINDOW_WIDTH as f64 / rect.width())) as f32,
        (offset_y * (GameOptions::WINDOW_HEIGHT as f64 / rect.height())) as f32,
    );

    window.set_cursor_position(Some(corrected));
}

fn window_resolution() -> WindowResolution {
    let resolution = WindowResolution::new(GameOptions::WINDOW_WIDTH, GameOptions::WINDOW_HEIGHT);

    // Pin logical == physical pixels so all UI layout math (tile/border sizes)
    // stays exact, regardless of the browser's devicePixelRatio/zoom. The
    // canvas is then scaled to fit the viewport via CSS, not by Bevy.
    #[cfg(target_arch = "wasm32")]
    let resolution = resolution.with_scale_factor_override(1.0);

    resolution
}
