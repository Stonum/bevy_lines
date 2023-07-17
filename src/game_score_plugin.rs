use bevy::prelude::*;

use crate::board_plugin::board::Board;
use crate::events::IncrementCurrentGameScore;

pub struct GameScorePlugin;

#[derive(Default, Resource)]
pub struct GameScore {
    pub current_score: u32,
    pub best_score: u32,
}

#[derive(Component)]
pub struct CurrentScore;

#[derive(Component)]
pub struct BestScore;

impl Plugin for GameScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameScore>()
            .add_startup_system(spawn_score_fields)
            .add_system(game_score_system)
            .add_system(render_score_text);
    }
}

fn spawn_score_fields(
    game: Res<GameScore>,
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

    let position_y = board.phisical_size / 2. + board.options.tile_size;
    let position_x = board.options.tile_size * 4.;

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!("{:0>5}", game.current_score),
                    text_style.clone(),
                )],
                ..Default::default()
            },
            transform: Transform::from_xyz(position_x, position_y, 0.),
            ..default()
        },
        CurrentScore,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!("{:0>5}", game.current_score),
                    text_style,
                )],
                ..Default::default()
            },
            transform: Transform::from_xyz(-position_x, position_y, 0.),
            ..default()
        },
        BestScore,
    ));
}

fn render_score_text(
    game: Res<GameScore>,
    mut q_curr_score: Query<&mut Text, With<CurrentScore>>,
    mut q_best_score: Query<&mut Text, (With<BestScore>, Without<CurrentScore>)>,
) {
    if game.is_changed() {
        for mut text in &mut q_curr_score {
            text.sections[0].value = format!("{:0>5}", game.current_score);
        }
        for mut text in &mut q_best_score {
            text.sections[0].value = format!("{:0>5}", game.best_score);
        }
    }
}

fn game_score_system(
    mut game: ResMut<GameScore>,
    mut ev_inc: EventReader<IncrementCurrentGameScore>,
) {
    for ev in ev_inc.iter() {
        game.current_score += ev.0;
    }
}
