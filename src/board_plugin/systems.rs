use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::cmp::Ordering;

use crate::events::IncrementCurrentGameScore;

use super::ball::*;
use super::board::*;
use super::events::*;
use super::next_balls::*;
use super::Coordinates;

pub fn spawn_board(
    mut board: ResMut<Board>,
    board_assets: Res<BoardAssets>,
    ball_assets: Res<BallAssets>,
    mut commands: Commands,
    mut ev_spawn_balls: EventWriter<SpawnNewBallEvent>,
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
                let ball_color = BallColor::new();
                ev_spawn_balls.send(SpawnNewBallEvent(ball_color));
            }
        })
        .insert(Name::new("Board"))
        .id();

    board.entity = Some(entity);
}

pub fn spawn_animation_timer(mut commands: Commands) {
    // timer for animate active ball
    commands.spawn(BallAnimattionTimer::default());
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
    mut board: ResMut<Board>,
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut q_balls: Query<(Entity, &mut Coordinates, &BallColor), With<Ball>>,
    query_next_ball: Query<&mut BallColor, (With<NextBall>, Without<Ball>)>,
    mut ev_spawn_balls: EventWriter<SpawnNewBallEvent>,
    mut ev_change_next: EventWriter<ChangeNextBallsEvent>,
    mut ev_inc_score: EventWriter<IncrementCurrentGameScore>,
) {
    let win = q_windows.get_single().expect("no primary window");
    // let mut board = query_board.get_single().expect("no board");

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(position) = win.cursor_position() {
            if let Some(coord) = board.logical_pos(win, position) {
                for (entity, ball_coord, _ball_color) in q_balls.iter() {
                    if *ball_coord == coord {
                        if let Some(entity) = board.active_ball {
                            commands.entity(entity).remove::<BallAnimationState>();
                            board.active_ball = None;
                        }
                        board.active_ball = Some(entity);
                        commands
                            .entity(entity)
                            .insert(BallAnimationState::default());
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

                            if let Some(entity) = board.active_ball {
                                commands.entity(entity).remove::<BallAnimationState>();
                                board.active_ball = None;
                            }

                            coordinates.0 = coord.0;
                            coordinates.1 = coord.1;

                            let despawned_balls = board.get_balls_for_despawn();

                            for line in despawned_balls {
                                // set game score
                                ev_inc_score.send(IncrementCurrentGameScore(line.len() as u32 * 2));

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

                            for color in query_next_ball.iter() {
                                // spawn new balls
                                ev_spawn_balls.send(SpawnNewBallEvent(*color));
                            }
                            // change next colors
                            ev_change_next.send(ChangeNextBallsEvent);
                        }
                    }
                }
            }
        }
    }
}

pub fn animate_ball_system(
    time: Res<Time>,
    mut query_animated_ball: Query<(&mut Transform, &mut BallAnimationState)>,
    mut query: Query<&mut BallAnimattionTimer>,
) {
    for (mut transform, mut state) in query_animated_ball.iter_mut() {
        for mut timer in &mut query {
            if timer.tick(time.delta()).just_finished() {
                match *state {
                    BallAnimationState::Up => {
                        transform.translation.y += 4.;
                        *state = BallAnimationState::Down;
                    }
                    BallAnimationState::Down => {
                        transform.translation.y -= 4.;
                        *state = BallAnimationState::Up;
                    }
                }
            }
        }
    }
}

pub fn spawn_new_ball(
    mut board: ResMut<Board>,
    ball_assets: Res<BallAssets>,
    mut commands: Commands,
    mut ev_spawn_balls: EventReader<SpawnNewBallEvent>,
) {
    for ev in ev_spawn_balls.iter() {
        if let Some(entity) = board.entity {
            let coord = board.get_free_tile().unwrap();
            let color = ev.0;
            board.tiles_map.insert(coord, Some(color));

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
                    .insert(color)
                    .insert(coord);
            });
        }
    }
}
