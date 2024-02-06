use bevy::prelude::*;

use crate::events::IncrementCurrentGameScore;
use crate::layout::{HeaderLeft, HeaderRight, MainLeft, MainRight};
use crate::leader_board_plugin::LeaderBoard;
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

#[derive(Component)]
struct LeaderName;

#[derive(Component)]
struct LeaderPodium;

#[derive(Component)]
struct ContenderPodium;

impl Plugin for GameScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameScore>()
            .add_systems(Startup, (spawn_score_fields, spawn_score_avatars))
            .add_systems(
                OnEnter(GameState::Playing),
                (init_game_score, init_leader_name),
            )
            .add_systems(
                Update,
                (game_score_system, render_score_text, podium_system),
            );
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

    let r_header = r_header.get_single().expect("Header right not found");
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

fn spawn_score_avatars(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    l_main: Query<Entity, With<MainLeft>>,
    r_main: Query<Entity, With<MainRight>>,
) {
    let font = asset_server.load("fonts/ThinPixel7.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::YELLOW_GREEN,
    };

    let l_main = l_main.get_single().expect("Main left not found");

    commands.entity(l_main).with_children(|main| {
        main.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(asset_server.load("leader.png")),
                ..default()
            });
            parent.spawn(ImageBundle {
                image: UiImage::new(asset_server.load("pillar_top.png")),
                ..default()
            });
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(asset_server.load("pillar.png")),
                    ..default()
                },
                LeaderPodium,
            ));
            parent.spawn(ImageBundle {
                image: UiImage::new(asset_server.load("pillar_bottom.png")),
                ..default()
            });
            parent.spawn((
                TextBundle {
                    text: Text::from_section("", text_style.clone()),
                    ..default()
                },
                LeaderName,
            ));
        });
    });

    let r_main = r_main.get_single().expect("Main right not found");

    commands.entity(r_main).with_children(|main| {
        main.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(asset_server.load("contender.png")),
                ..default()
            });
            parent.spawn(ImageBundle {
                image: UiImage::new(asset_server.load("pillar_top.png")),
                ..default()
            });
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(asset_server.load("pillar.png")),
                    ..default()
                },
                ContenderPodium,
            ));
            parent.spawn(ImageBundle {
                image: UiImage::new(asset_server.load("pillar_bottom.png")),
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section("Contender", text_style),
                ..default()
            });
        });
    });
}

fn init_game_score(mut game_score: ResMut<GameScore>, leaders: Res<LeaderBoard>) {
    game_score.current_score = 0;
    if let Some(score) = leaders.get_best_score() {
        game_score.best_score = score;
    }
}

fn init_leader_name(
    leaders: Res<LeaderBoard>,
    mut q_leader_name: Query<&mut Text, With<LeaderName>>,
) {
    if let Some(leader) = leaders.get_best_player() {
        for mut text in &mut q_leader_name {
            text.sections[0].value = format!("{leader}");
        }
    }
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
    if ev_inc.len() > 0 {
        game.current_score += ev_inc.iter().map(|ev| ev.0).sum::<u32>();
    }
}

fn podium_system(
    game: Res<GameScore>,
    mut q_leader_podium: Query<&mut Style, With<LeaderPodium>>,
    mut q_contender_podium: Query<&mut Style, (With<ContenderPodium>, Without<LeaderPodium>)>,
) {
    if game.is_changed() {
        let mut leader = q_leader_podium.single_mut();
        let mut contender = q_contender_podium.single_mut();

        leader.height = Val::Percent(100.0);
        contender.height = Val::Percent(0.0);

        if game.current_score < game.best_score {
            contender.height =
                Val::Percent(game.current_score as f32 / game.best_score as f32 * 100.0);
        } else {
            leader.height =
                Val::Percent(game.best_score as f32 / game.current_score as f32 * 100.0);
        }
    }
}
