use bevy::prelude::*;

use super::ball::BallAssets;
use super::ball::BallColor;
use super::board::BoardAssets;
use super::events::ChangeNextBallsEvent;
use crate::GameOptions;
use crate::GameState;

pub struct NextBallsPlugin;

impl Plugin for NextBallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_next_board)
            .add_systems(OnEnter(GameState::Playing), spawn_next_balls)
            .add_systems(Update, (render_next_color, change_next_color))
            .add_systems(OnExit(GameState::Playing), despawn_next_balls);
    }
}

#[derive(Component)]
struct NextBoard;

#[derive(Component)]
struct NextTile;

#[derive(Debug, Component)]
pub struct NextBall {
    pub color: BallColor,
}

impl NextBall {
    fn new() -> Self {
        Self {
            color: BallColor::new(),
        }
    }
    fn change_color(&mut self) {
        self.color = BallColor::new()
    }
}

fn spawn_next_board(board_assets: Res<BoardAssets>, mut commands: Commands) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: board_assets.board_color,
                    custom_size: Some(Vec2::new(
                        GameOptions::TILE_SIZE * 3.0,
                        GameOptions::TILE_SIZE,
                    )),
                    ..default()
                },
                transform: Transform::from_xyz(
                    0.,
                    GameOptions::BOARD_SIZE / 2. + GameOptions::TILE_SIZE,
                    0.,
                ),
                ..default()
            },
            NextBoard,
        ))
        .with_children(|parent| {
            for x in 0..3 {
                let position_x: f32 = -GameOptions::TILE_SIZE + GameOptions::TILE_SIZE * x as f32;

                // spawn tiles
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: board_assets.tile_color,
                            custom_size: Some(Vec2::splat(
                                GameOptions::TILE_SIZE - GameOptions::TILE_PADDING,
                            )),
                            ..default()
                        },
                        transform: Transform::from_xyz(position_x, 0., 1.),
                        ..default()
                    },
                    NextTile,
                ));
            }
        });
}

fn spawn_next_balls(
    ball_assets: Res<BallAssets>,
    mut commands: Commands,
    q_next_tiles: Query<Entity, With<NextTile>>,
) {
    for entity in q_next_tiles.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: ball_assets.texture.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(GameOptions::BALL_SIZE)),
                        ..default()
                    },
                    ..default()
                },
                NextBall::new(),
            ));
        });
    }
}

fn render_next_color(mut query_next_ball: Query<(&NextBall, &mut Sprite), Changed<NextBall>>) {
    for (ball, mut sprite) in query_next_ball.iter_mut() {
        sprite.color = ball.color.into();
    }
}

fn change_next_color(
    mut query_next_ball: Query<&mut NextBall, With<NextBall>>,
    mut ev_change_next: EventReader<ChangeNextBallsEvent>,
) {
    for _ in ev_change_next.iter() {
        for mut ball in query_next_ball.iter_mut() {
            ball.change_color();
        }
    }
}

fn despawn_next_balls(mut commands: Commands, q_next_ball: Query<Entity, With<NextBall>>) {
    for entity in q_next_ball.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
