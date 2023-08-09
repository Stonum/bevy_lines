mod ball;
pub mod board;
mod events;
mod next_balls_plugin;
mod systems;

use bevy::prelude::*;

use ball::BallAssets;
use board::Board;
use board::BoardAssets;

use next_balls_plugin::NextBallsPlugin;

use events::*;
use systems::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallAssets>()
            .init_resource::<BoardAssets>()
            .init_resource::<Board>();

        app.add_plugin(NextBallsPlugin);

        app.add_event::<SpawnNewBallEvent>();
        app.add_event::<ChangeNextBallsEvent>();

        app.add_startup_system(spawn_board)
            .add_startup_system(spawn_animation_timer)
            .add_system(render_balls)
            .add_system(animate_ball_system)
            .add_system(spawn_new_ball)
            .add_system(handle_mouse_clicks);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoardOptions {
    pub tile_size: f32,
    pub tile_padding: f32,
    pub tile_count: u8,
    pub ball_size: f32,
    pub min_balls_on_line: usize,
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            tile_size: 45.0,
            tile_padding: 5.0,
            tile_count: 9,
            ball_size: 35.0,
            min_balls_on_line: 5,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Component)]
#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
pub struct Coordinates(pub u8, pub u8);
