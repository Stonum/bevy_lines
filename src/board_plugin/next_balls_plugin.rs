use bevy::prelude::*;

use super::ball::BallAssets;
use super::ball::BallColor;
use super::events::ChangeNextBallsEvent;
use crate::layout::HeaderCenter;
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

fn spawn_next_board(mut commands: Commands, header: Query<Entity, With<HeaderCenter>>) {
    let header = header.get_single().expect("Header not found");

    commands.entity(header).with_children(|header| {
        for _ in 0..3 {
            header.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(GameOptions::TILE_SIZE),
                        height: Val::Px(GameOptions::TILE_SIZE),
                        border: UiRect::all(Val::Px(GameOptions::TILE_PADDING)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: GameOptions::BOARD_COLOR.into(),
                    background_color: GameOptions::TILE_COLOR.into(),
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
                ImageBundle {
                    style: Style {
                        width: Val::Px(GameOptions::BALL_SIZE),
                        height: Val::Px(GameOptions::BALL_SIZE),
                        ..default()
                    },
                    image: UiImage::new(ball_assets.texture.clone()),
                    ..default()
                },
                NextBall::new(),
            ));
        });
    }
}

fn render_next_color(
    mut query_next_ball: Query<(&NextBall, &mut BackgroundColor), Changed<NextBall>>,
) {
    for (ball, mut color) in query_next_ball.iter_mut() {
        color.0 = ball.color.into();
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
