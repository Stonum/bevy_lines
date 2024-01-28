mod ball;
pub mod board;
mod events;
mod next_balls_plugin;
mod systems;

use bevy::prelude::*;

use ball::BallAssets;
use board::Board;

use next_balls_plugin::NextBallsPlugin;

use events::*;
use systems::*;

use crate::GameOptions;
use crate::GameState;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallAssets>().init_resource::<Board>();

        app.add_plugins(NextBallsPlugin);

        app.add_event::<SpawnNewBallEvent>();
        app.add_event::<ChangeNextBallsEvent>();

        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_board, spawn_animation_timer),
        )
        .add_systems(
            Update,
            (
                render_balls,
                animate_ball_system,
                spawn_new_ball,
                handle_mouse_clicks,
            ),
        )
        .add_systems(OnExit(GameState::Playing), despawn_board);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Component)]
pub struct Coordinates(pub u8, pub u8);

impl TryFrom<Vec2> for Coordinates {
    type Error = &'static str;
    fn try_from(pos: Vec2) -> Result<Self, Self::Error> {
        let window_size = Vec2::new(GameOptions::WINDOW_WIDTH, GameOptions::WINDOW_HEIGHT);
        let position = pos - window_size / 2.;
        let size = GameOptions::BOARD_SIZE / 2.;

        if size < position.x.abs() || size < position.y.abs() {
            return Err("Cursor position out of bounds");
        }

        let coord = Coordinates(
            ((position.x + size) / GameOptions::TILE_SIZE) as u8,
            ((position.y + size) / GameOptions::TILE_SIZE) as u8,
        );
        Ok(coord)
    }
}

impl From<Coordinates> for Vec2 {
    fn from(coord: Coordinates) -> Self {
        let offset = -GameOptions::BOARD_SIZE / 2.;
        Vec2::new(
            (coord.0 as f32 * GameOptions::TILE_SIZE) + (GameOptions::TILE_SIZE / 2.) + offset,
            -((coord.1 as f32 * GameOptions::TILE_SIZE) + (GameOptions::TILE_SIZE / 2.) + offset),
        )
    }
}

#[derive(Component)]
pub struct BoardTile;
