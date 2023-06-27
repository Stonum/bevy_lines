use crate::components::*;
use crate::events::*;
use crate::{BALL_SIZE, BOARD_COLOR, TILE_COLOR, TILE_COUNT, TILE_PADDING, TILE_SIZE};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use rand::seq::SliceRandom;

pub fn spawn_board(
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: TILE_COLOR,
                        custom_size: Some(Vec2::splat(TILE_SIZE - TILE_PADDING)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.phisical_pos(coord.0),
                        board.phisical_pos(coord.1),
                        1.0,
                    ),
                    ..default()
                });
            }

            // spawn startup balls
            for _ in 0..3 {
                let coord = board.get_free_tile().unwrap();
                let ball_color = BallColor::new();

                board.tiles_map.insert(coord, Some(ball_color));

                parent
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Circle::new(BALL_SIZE / 2.0).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(ball_color.0)),
                        transform: Transform::from_xyz(
                            board.phisical_pos(coord.0),
                            board.phisical_pos(coord.1),
                            2.,
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

pub fn spawn_next_board(
    board: Res<Board>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: TILE_COLOR,
                        custom_size: Some(Vec2::splat(TILE_SIZE - TILE_PADDING)),
                        ..default()
                    },
                    transform: Transform::from_xyz(position_x, 0., 1.),
                    ..default()
                });

                let ball_color = BallColor::new();
                parent
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Circle::new(BALL_SIZE / 2.0).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(ball_color.0)),
                        transform: Transform::from_xyz(position_x, 0., 2.),
                        ..default()
                    })
                    .insert(NextBall)
                    .insert(ball_color);
            }
        });
}

pub fn change_next_color(
    mut query_next_ball: Query<&mut BallColor, With<NextBall>>,
    mut ev_change_next: EventReader<ChangeNextBalls>,
) {
    for ev in ev_change_next.iter() {
        println!("Change next color");
        for mut color in query_next_ball.iter_mut() {
            color.0 = BallColor::new().0;
        }
    }
}

pub fn render_next_balls(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<
        (&BallColor, &mut Handle<ColorMaterial>),
        (Changed<BallColor>, With<NextBall>),
    >,
) {
    for (color, mut handle) in query.iter_mut() {
        *handle = materials.add(ColorMaterial::from(color.0));
    }
}

pub fn render_balls(
    board: Res<Board>,
    mut query: Query<(&Coordinates, &mut Transform), (Changed<Coordinates>, With<Ball>)>,
) {
    for (coord, mut transform) in query.iter_mut() {
        info!("Move ball to {coord:?}");
        transform.translation.x = board.phisical_pos(coord.0);
        transform.translation.y = board.phisical_pos(coord.1);
    }
}

pub fn handle_mouse_clicks(
    mouse_input: Res<Input<MouseButton>>,
    mut board: ResMut<Board>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut q_balls: Query<(Entity, &mut Coordinates), With<Ball>>,
    mut ev_spawn_balls: EventWriter<SpawnBallsEvent>,
    mut ev_change_next: EventWriter<ChangeNextBalls>,
) {
    let win = q_windows.get_single().expect("no primary window");
    // let mut board = query_board.get_single().expect("no board");

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(position) = win.cursor_position() {
            if let Some(coord) = board.logical_pos(win, position) {
                for (entity, ball_coord) in q_balls.iter() {
                    if *ball_coord == coord {
                        board.active_ball = Some(entity);
                    }
                }
                // move ball to new position
                if let Some(ball) = board.active_ball {
                    if let Ok((_, ref mut coordinates)) = q_balls.get_mut(ball) {
                        if coordinates.0 != coord.0 || coordinates.1 != coord.1 {
                            coordinates.0 = coord.0;
                            coordinates.1 = coord.1;
                            board.active_ball = None;

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    query_next_ball: Query<&mut BallColor, With<NextBall>>,
    mut ev_spawn_balls: EventReader<SpawnBallsEvent>,
) {
    for _ in ev_spawn_balls.iter() {
        println!("Spawn new balls");
        if let Some(entity) = board.entity {
            for color in query_next_ball.iter() {
                let coord = board.get_free_tile().unwrap();
                board.tiles_map.insert(coord, Some(*color));

                commands.entity(entity).with_children(|parent| {
                    parent
                        .spawn(MaterialMesh2dBundle {
                            mesh: meshes
                                .add(shape::Circle::new(BALL_SIZE / 2.0).into())
                                .into(),
                            material: materials.add(ColorMaterial::from(color.0)),
                            transform: Transform::from_xyz(
                                board.phisical_pos(coord.0),
                                board.phisical_pos(coord.1),
                                2.,
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
