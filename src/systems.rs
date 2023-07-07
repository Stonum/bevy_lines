use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::cmp::Ordering;

use crate::components::*;
use crate::events::*;
use crate::game_score::GameScore;
use crate::next_balls::*;
use crate::resources::*;

pub fn spawn_board(
    mut board: ResMut<Board>,
    board_assets: Res<BoardAssets>,
    ball_assets: Res<BallAssets>,
    mut commands: Commands,
) {
    let entity = commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color: board_assets.board_color,
                custom_size: Some(Vec2::splat(board.phisical_size)),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            for coord in board.tiles_map.keys() {
                let position = board.phisical_pos(&coord);
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.tile_color,
                        custom_size: Some(Vec2::splat(
                            board.options.tile_size - board.options.tile_padding,
                        )),
                        ..default()
                    },
                    transform: Transform::from_translation(position.extend(1.)),
                    ..default()
                });
            }

            // spawn startup balls
            for _ in 0..3 {
                let coord = board.get_free_tile().unwrap();
                let ball_color = BallColor::new();

                board.tiles_map.insert(coord, Some(ball_color));

                parent
                    .spawn(SpriteBundle {
                        texture: ball_assets.texture.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(board.options.ball_size)),
                            color: ball_color.get(),
                            ..default()
                        },
                        transform: Transform::from_translation(
                            board.phisical_pos(&coord).extend(2.),
                        ),
                        ..default()
                    })
                    .insert(Name::from("Ball"))
                    .insert(Ball)
                    .insert(ball_color)
                    .insert(coord);
            }
        })
        .insert(Name::new("Board"))
        .id();

    board.entity = Some(entity);
}

pub fn render_balls(
    board: Res<Board>,
    mut query: Query<(&Coordinates, &mut Transform), (Changed<Coordinates>, With<Ball>)>,
) {
    for (coord, mut transform) in query.iter_mut() {
        info!("Move ball to {coord:?}");
        let Vec2 { x, y } = board.phisical_pos(&coord);
        transform.translation.x = x;
        transform.translation.y = y;
    }
}

pub fn handle_mouse_clicks(
    mouse_input: Res<Input<MouseButton>>,
    mut game: ResMut<GameScore>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut q_balls: Query<(Entity, &mut Coordinates, &BallColor), With<Ball>>,
    mut ev_spawn_balls: EventWriter<SpawnBallsEvent>,
) {
    let win = q_windows.get_single().expect("no primary window");
    // let mut board = query_board.get_single().expect("no board");

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(position) = win.cursor_position() {
            if let Some(coord) = board.logical_pos(win, position) {
                for (entity, ball_coord, _ball_color) in q_balls.iter() {
                    if *ball_coord == coord {
                        board.active_ball = Some(entity);
                    }
                }
                // move ball to new position
                if let Some(ball) = board.active_ball {
                    if let Ok((_entity, mut coordinates, color)) = q_balls.get_mut(ball) {
                        if coordinates.partial_cmp(&coord) != Some(Ordering::Equal) {
                            // remove ball from coordinates
                            board.tiles_map.insert(*coordinates, None);
                            // move ball to coord
                            board.tiles_map.insert(coord, Some(*color));

                            board.active_ball = None;

                            coordinates.0 = coord.0;
                            coordinates.1 = coord.1;

                            let despawned_balls = board.get_balls_for_despawn();

                            for line in despawned_balls {
                                // set game score
                                game.current_score += line.len() as u32 * 2;

                                for coord in line {
                                    let ball = q_balls
                                        .iter_mut()
                                        .find(|(_, ball_coord, ..)| (*ball_coord).clone() == coord);

                                    if let Some((entity, ..)) = ball {
                                        board.tiles_map.insert(coord, None);
                                        commands.entity(entity).despawn_recursive();
                                    }
                                }
                            }

                            // spawn new balls
                            ev_spawn_balls.send(SpawnBallsEvent);
                        }
                    }
                }
            }
        }
    }
}

pub fn animate_ball_system(mut _commands: Commands, board: Res<Board>) {
    if let Some(_ball) = board.active_ball {

        // commands.entity(ball)
    }
}

pub fn spawn_new_balls(
    mut board: ResMut<Board>,
    ball_assets: Res<BallAssets>,
    mut commands: Commands,
    query_next_ball: Query<&mut BallColor, With<NextBall>>,
    mut ev_spawn_balls: EventReader<SpawnBallsEvent>,
    mut ev_change_next: EventWriter<ChangeNextBalls>,
) {
    for _ in ev_spawn_balls.iter() {
        info!("Spawn new balls");
        if let Some(entity) = board.entity {
            for color in query_next_ball.iter() {
                let coord = board.get_free_tile().unwrap();
                board.tiles_map.insert(coord, Some(*color));

                commands.entity(entity).with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            texture: ball_assets.texture.clone(),
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(board.options.ball_size)),
                                color: color.get(),
                                ..default()
                            },
                            transform: Transform::from_translation(
                                board.phisical_pos(&coord).extend(2.),
                            ),
                            ..default()
                        })
                        .insert(Name::from("Ball"))
                        .insert(Ball)
                        .insert(*color)
                        .insert(coord);
                });
            }
            // change next colors
            ev_change_next.send(ChangeNextBalls);
        }
    }
}
