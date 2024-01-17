use bevy::ecs::component::Component;
use bevy::prelude::*;

use crate::GameOptions;
use crate::GameState;

const NORMAL_BUTTON: Color = GameOptions::TILE_COLOR;
const HOVERED_BUTTON: Color = Color::rgb(0.80, 0.80, 0.80);
const PRESSED_BUTTON: Color = Color::rgb(0.90, 0.90, 0.90);
const BUTTON_BORDER_COLOR: Color = GameOptions::BOARD_COLOR;

const BUTTON_HEIGHT: f32 = GameOptions::TILE_SIZE;
const BUTTON_WIDTH: f32 = BUTTON_HEIGHT * 5.0;
const BUTTON_BORDER: f32 = GameOptions::TILE_PADDING / 2.0;
const BUTTON_PADDING: f32 = GameOptions::TILE_PADDING * 2.0;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Restarting), start_game)
            .add_systems(Update, button_system);
    }
}

#[derive(Component)]
enum MenuButton {
    Restart,
    Leaderboard,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/ThinPixel7.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 35.0,
        color: Color::DARK_GRAY,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::End,
                padding: UiRect::bottom(Val::Px(BUTTON_PADDING)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            spawn_button(parent, &text_style, "Restart", MenuButton::Restart);
            spawn_button(parent, &text_style, "Leaderboard", MenuButton::Leaderboard);
        });
}

fn spawn_button(
    parent: &mut ChildBuilder,
    text_style: &TextStyle,
    text: &str,
    comp: impl Component,
) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(BUTTON_WIDTH),
                height: Val::Px(BUTTON_HEIGHT),
                border: UiRect::all(Val::Px(BUTTON_BORDER)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                margin: UiRect::horizontal(Val::Px(BUTTON_BORDER)),
                ..default()
            },
            border_color: BorderColor(BUTTON_BORDER_COLOR),
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(text, text_style.clone()));
        })
        .insert(comp);
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<MenuButton>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match *button_type {
                    MenuButton::Restart => game_state.set(GameState::Restarting),
                    MenuButton::Leaderboard => game_state.set(GameState::Leaderboard),
                }
            }
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
}

fn start_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Playing);
}
