use std::cmp::Ordering;

use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::{BALL_SIZE, BOARD_COLOR, TILE_COLOR, TILE_PADDING, TILE_SIZE};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_board(mut board: ResMut<Board>, ball_assets: Res<BallAssets>, mut commands: Commands) {
    let entity = commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color: BOARD_COLOR,
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
                        color: TILE_COLOR,
                        custom_size: Some(Vec2::splat(TILE_SIZE - TILE_PADDING)),
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
                            custom_size: Some(Vec2::splat(BALL_SIZE)),
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

pub fn spawn_next_board(board: Res<Board>, ball_assets: Res<BallAssets>, mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: BOARD_COLOR,
                custom_size: Some(Vec2::new(TILE_SIZE * 3.0, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(0., board.phisical_size / 2. + TILE_SIZE, 0.),
            ..default()
        })
        .insert(Name::new("NextBoard"))
        .insert(NextBoard)
        .with_children(|parent| {
            for x in 0..3 {
                let position_x: f32 = -TILE_SIZE + TILE_SIZE * x as f32;

                // spawn tiles
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: TILE_COLOR,
                        custom_size: Some(Vec2::splat(TILE_SIZE - TILE_PADDING)),
                        ..default()
                    },
                    transform: Transform::from_xyz(position_x, 0., 1.),
                    ..default()
                });

                // spawn startup colors
                let ball_color = BallColor::new();
                parent
                    .spawn(SpriteBundle {
                        texture: ball_assets.texture.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(BALL_SIZE)),
                            color: ball_color.get(),
                            ..default()
                        },
                        transform: Transform::from_xyz(position_x, 0., 2.),
                        ..default()
                    })
                    .insert(NextBall)
                    .insert(ball_color);
            }
        });
}

pub fn spawn_score_fields(
    game: Res<Game>,
    board: Res<Board>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let font = asset_server.load("fonts/Glitch-Demo.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::GREEN,
    };
    let text_alignment = TextAlignment::Center;

    let position_y = board.phisical_size / 2. + TILE_SIZE;
    let position_x = TILE_SIZE * 4.;

    commands
        .spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(format!("{:0>5}", game.score), text_style)],
                ..Default::default()
            },
            transform: Transform::from_xyz(position_x, position_y, 0.),
            ..default()
        })
        .insert(ScoreText);
}

pub fn change_next_color(
    mut query_next_ball: Query<&mut BallColor, With<NextBall>>,
    mut ev_change_next: EventReader<ChangeNextBalls>,
) {
    for _ in ev_change_next.iter() {
        info!("Change next color");
        for mut color in query_next_ball.iter_mut() {
            *color = BallColor::new();
        }
    }
}

pub fn render_next_balls(
    mut query: Query<(&BallColor, &mut Sprite), (Changed<BallColor>, With<NextBall>)>,
) {
    for (color, mut sprite) in query.iter_mut() {
        sprite.color = color.get();
    }
}

pub fn render_score_text(game: Res<Game>, mut query: Query<&mut Text, With<ScoreText>>) {
    if game.is_changed() {
        for mut text in &mut query {
            text.sections[0].value = format!("{:0>5}", game.score);
        }
    }
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
    mut game: ResMut<Game>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut q_balls: Query<(Entity, &mut Coordinates, &BallColor), With<Ball>>,
    mut ev_spawn_balls: EventWriter<SpawnBallsEvent>,
    mut ev_change_next: EventWriter<ChangeNextBalls>,
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
                                game.score += line.len() as u32 * 2;

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
                            // change next colors
                            ev_change_next.send(ChangeNextBalls);
                        }
                    }
                }
            }
        }
    }
}

pub fn animate_ball_system(mut commands: Commands, board: Res<Board>) {
    if let Some(ball) = board.active_ball {

        // commands.entity(ball)
    }
}

pub fn spawn_new_balls(
    mut board: ResMut<Board>,
    ball_assets: Res<BallAssets>,
    mut commands: Commands,
    query_next_ball: Query<&mut BallColor, With<NextBall>>,
    mut ev_spawn_balls: EventReader<SpawnBallsEvent>,
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
                                custom_size: Some(Vec2::splat(BALL_SIZE)),
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
        }
    }
}
