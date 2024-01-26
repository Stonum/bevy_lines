use bevy::prelude::*;

use crate::events::IncrementCurrentGameScore;
use crate::layout::{HeaderLeft, HeaderRight};
use crate::leader_board_plugin::LeaderBoard;
use crate::GameOptions;
use crate::GameState;

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
            .add_systems(Startup, spawn_score_fields)
            .add_systems(OnEnter(GameState::Playing), init_game_score)
            .add_systems(Update, (game_score_system, render_score_text));
    }
}

fn spawn_score_fields(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    l_header: Query<Entity, With<HeaderLeft>>,
    r_header: Query<Entity, With<HeaderRight>>,
) {
    let font = asset_server.load("fonts/Glitch-Demo.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::GREEN,
    };

    let l_header = l_header.get_single().expect("Header left not found");
    let r_header = r_header.get_single().expect("Header right not found");

    commands.entity(l_header).with_children(|header| {
        header.spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new("", text_style.clone())],
                    ..default()
                },
                ..default()
            },
            BestScore,
        ));
    });

    commands.entity(r_header).with_children(|header| {
        header.spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new("", text_style)],
                    ..default()
                },
                ..default()
            },
            CurrentScore,
        ));
    });
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

fn init_game_score(mut game_score: ResMut<GameScore>, leaders: Res<LeaderBoard>) {
    game_score.current_score = 0;
    if let Some(score) = leaders.get_best_score() {
        game_score.best_score = score;
    }
}

fn game_score_system(
    mut game: ResMut<GameScore>,
    mut ev_inc: EventReader<IncrementCurrentGameScore>,
) {
    if ev_inc.len() > 0 {
        game.current_score += ev_inc.iter().map(|ev| ev.0).sum::<u32>();
    }
}
