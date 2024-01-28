use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::cmp::Ordering;

use crate::events::IncrementCurrentGameScore;
use crate::layout::Main;
use crate::GameOptions;
use crate::GameState;

use super::ball::*;
use super::board::*;
use super::events::*;
use super::next_balls_plugin::*;
use super::BoardTile;
use super::Coordinates;

pub fn spawn_board(
    mut board: ResMut<Board>,
    mut commands: Commands,
    main: Query<Entity, With<Main>>,
) {
    let main = main.get_single().expect("Main not found");

    commands.entity(main).with_children(|main| {
        let mut board_bundle = main.spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_auto_flow: GridAutoFlow::Column,

                width: Val::Px(GameOptions::BOARD_SIZE),
                height: Val::Px(GameOptions::BOARD_SIZE),

                grid_template_columns: vec![GridTrack::flex(1.0); 9],
                grid_template_rows: vec![GridTrack::flex(1.0); 9],

                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: GameOptions::BOARD_COLOR.into(),
            ..default()
        });

        // board tiles
        board_bundle.with_children(|parent| {
            let mut coordinates: Vec<&Coordinates> = board.tiles_map.keys().into_iter().collect();
            coordinates.sort();

            for coordinate in coordinates {
                parent
                    .spawn(NodeBundle {
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
                    })
                    .insert(*coordinate)
                    .insert(BoardTile);
            }
        });
    });
}

pub fn spawn_startup_balls(mut ev_spawn_balls: EventWriter<SpawnNewBallEvent>) {
    // spawn startup balls
    for _ in 0..3 {
        let ball_color = BallColor::new();
        ev_spawn_balls.send(SpawnNewBallEvent(ball_color));
    }
}

pub fn spawn_animation_timer(mut commands: Commands) {
    // timer for animate active ball
    commands.spawn(BallAnimationTimer::default());
}

pub fn render_balls(
    mut query: Query<(&Coordinates, &mut Transform), (Changed<Coordinates>, With<Ball>)>,
) {
    for (coord, mut transform) in query.iter_mut() {
        let Vec2 { x, y } = Vec2::from(*coord);
        transform.translation.x = x;
        transform.translation.y = y;
    }
}

pub fn handle_mouse_clicks(
    mouse_input: Res<Input<MouseButton>>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut q_balls: Query<(&mut Coordinates, &mut Style), With<Ball>>,
    query_next_ball: Query<&NextBall, With<NextBall>>,
    query_tile: Query<(&Coordinates, Entity), (With<BoardTile>, Without<Ball>)>,
    mut ev_spawn_balls: EventWriter<SpawnNewBallEvent>,
    mut ev_change_next: EventWriter<ChangeNextBallsEvent>,
    mut ev_inc_score: EventWriter<IncrementCurrentGameScore>,
) {
    let win = q_windows.get_single().expect("no primary window");

    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    let next_coordinates: Option<Coordinates> = win
        .cursor_position()
        .and_then(|position| position.try_into().ok());

    if let Some(next_coordinates) = next_coordinates {
        let ball = board.tiles_map.get(&next_coordinates).unwrap().as_ref();
        match (board.active_ball, ball) {
            // set active ball
            (None, Some(ball)) => {
                commands
                    .entity(ball.entity)
                    .insert(BallAnimationState::default());
                board.active_ball = Some(ball.entity);
            }
            // change active ball
            (Some(active_ball), Some(ball)) if active_ball != ball.entity => {
                commands.entity(active_ball).remove::<BallAnimationState>();
                // fix ball position after stop animation
                if let Ok((_, mut style)) = q_balls.get_mut(active_ball) {
                    style.top = Val::Auto;
                }
                commands
                    .entity(ball.entity)
                    .insert(BallAnimationState::default());
                board.active_ball = Some(ball.entity);
            }
            // move active ball to new position
            (Some(active_ball), None) => {
                if let Ok((mut coordinates, mut style)) = q_balls.get_mut(active_ball) {
                    if coordinates.partial_cmp(&next_coordinates) != Some(Ordering::Equal)
                        && board
                            .get_path_to_move(&coordinates, &next_coordinates)
                            .is_some()
                    {
                        // remove ball from old coordinates
                        let ball_entity =
                            board.tiles_map.insert(*coordinates, None).unwrap_or(None);
                        // insert ball to new coordinates
                        board.tiles_map.insert(next_coordinates, ball_entity);

                        commands.entity(active_ball).remove::<BallAnimationState>();
                        style.top = Val::Auto;

                        // set new parent tile for ball
                        for (tile_coord, tile_entity) in query_tile.iter() {
                            if *tile_coord == next_coordinates {
                                commands.entity(active_ball).set_parent(tile_entity);
                            }
                        }

                        board.active_ball = None;

                        // change coordinates
                        coordinates.0 = next_coordinates.0;
                        coordinates.1 = next_coordinates.1;

                        let despawned_lines = despawn_balls_and_inc_score(
                            &mut board,
                            &mut commands,
                            &mut ev_inc_score,
                        );

                        if despawned_lines == 0 {
                            // spawn new balls
                            query_next_ball.iter().for_each(|next_ball| {
                                ev_spawn_balls.send(SpawnNewBallEvent(next_ball.color));
                            });
                            // mb new combinations after spawn new balls
                            despawn_balls_and_inc_score(
                                &mut board,
                                &mut commands,
                                &mut ev_inc_score,
                            );

                            // change next colors
                            ev_change_next.send(ChangeNextBallsEvent);
                        }
                    }
                }
            }
            // do nothing
            _ => (),
        }
    }
}

fn despawn_balls_and_inc_score(
    board: &mut ResMut<Board>,
    commands: &mut Commands,
    ev_inc_score: &mut EventWriter<IncrementCurrentGameScore>,
) -> usize {
    let despawned_balls = board.get_balls_for_despawn();
    let len = despawned_balls.len();

    for line in despawned_balls {
        // set game score
        ev_inc_score.send(IncrementCurrentGameScore(line.len() as u32 * 2));

        for coordinates in line {
            let ball = board.tiles_map.insert(coordinates, None).unwrap_or(None);
            if let Some(ball) = ball {
                commands.entity(ball.entity).despawn_recursive();
            }
        }
    }
    len
}

pub fn animate_ball_system(
    time: Res<Time>,
    mut query_animated_ball: Query<(&mut Style, &mut BallAnimationState)>,
    mut query_timer: Query<&mut BallAnimationTimer>,
) {
    for ((mut style, mut state), mut timer) in
        query_animated_ball.iter_mut().zip(query_timer.iter_mut())
    {
        if timer.tick(time.delta()).just_finished() {
            match *state {
                BallAnimationState::Up => {
                    style.top = Val::Px(3.0);
                    *state = BallAnimationState::Down;
                }
                BallAnimationState::Down => {
                    style.top = Val::Px(-3.0);
                    *state = BallAnimationState::Up;
                }
            }
        }
    }
}

pub fn spawn_new_ball(
    mut board: ResMut<Board>,
    ball_assets: Res<BallAssets>,
    mut commands: Commands,
    q_board_tile: Query<(&Coordinates, Entity), With<BoardTile>>,
    mut ev_spawn_balls: EventReader<SpawnNewBallEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for SpawnNewBallEvent(color) in ev_spawn_balls.iter() {
        let coord = match board.get_free_tile() {
            Some(free_coordinates) => free_coordinates,
            None => {
                game_state.set(GameState::GameOver);
                return;
            }
        };

        if let Some((_, tile)) = q_board_tile.iter().find(|(c, _)| **c == coord) {
            commands.entity(tile).with_children(|parent| {
                let entity = parent
                    .spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(GameOptions::BALL_SIZE),
                            height: Val::Px(GameOptions::BALL_SIZE),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::from(*color)),
                        image: UiImage::new(ball_assets.texture.clone()),
                        ..default()
                    })
                    .insert(Ball)
                    .insert(*color)
                    .insert(coord)
                    .id();

                board
                    .tiles_map
                    .insert(coord, Some(BallEntity::new(*color, entity)));
            });
        }

        if board.get_free_tile().is_none() {
            game_state.set(GameState::GameOver);
        };
    }
}

pub fn despawn_board_balls(
    mut commands: Commands,
    mut board: ResMut<Board>,
    query_balls: Query<Entity, &Ball>,
) {
    for entity in query_balls.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for (_coord, ball) in board.tiles_map.iter_mut() {
        *ball = None;
    }
}
