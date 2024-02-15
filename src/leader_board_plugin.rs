use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use js_sys::JSON;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::game_score_plugin::GameScore;
use crate::layout::MainCenter;
use crate::GameOptions;
use crate::GameState;
use crate::LeaderBoardState;

const LINE_COLOR: Color = GameOptions::TILE_COLOR;
const LINE_BORDER_COLOR: Color = GameOptions::BOARD_COLOR;

const LINE_HEIGHT: f32 = GameOptions::TILE_SIZE;
const LINE_WIDTH: f32 = GameOptions::BOARD_SIZE;
const LINE_BORDER: f32 = GameOptions::TILE_PADDING;
const LINE_PADDING: f32 = GameOptions::TILE_PADDING * 4.0;

const MAX_PLAYERS: usize = 10;

pub struct LeaderBoardPlugin;

impl Plugin for LeaderBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LeaderBoard::new())
            .add_systems(OnEnter(GameState::GameOver), change_leaders)
            .add_systems(OnEnter(LeaderBoardState::Show), spawn_leader_board)
            .add_systems(OnExit(LeaderBoardState::Show), despawn_leader_board);
    }
}

#[derive(Resource, Debug)]
pub struct LeaderBoard {
    pub players: Vec<(String, u32)>,
}

#[derive(Component)]
struct LeaderBoardNode;

impl LeaderBoard {
    pub fn new() -> Self {
        let mut players: Vec<(String, u32)> = match Self::get_from_local_storage() {
            Some(players) => players,
            None => (1..=MAX_PLAYERS)
                .into_iter()
                .map(|x| ("Player ".to_string() + &x.to_string(), (x * 100) as u32))
                .collect(),
        };

        players.sort_by_key(|x| !x.1); // reversed sorting
        players.truncate(MAX_PLAYERS);
        Self { players }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn get_from_local_storage() -> Option<Vec<(String, u32)>> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    fn get_from_local_storage() -> Option<Vec<(String, u32)>> {
        let window = web_sys::window()?;
        let local_storage = window.local_storage().ok()??;
        let mut players = Vec::with_capacity(10);

        if let Ok(Some(leader_board)) = local_storage.get_item("leader_board") {
            let data = JSON::parse(&leader_board).ok()?;
            let iter = js_sys::try_iter(&data).ok()??;
            for item in iter {
                let item = item.ok()?;
                let item_array: &js_sys::Array = wasm_bindgen::JsCast::dyn_ref(&item)?;
                let name = item_array.shift().as_string();
                let score = item_array.shift().as_f64();
                if let (Some(name), Some(score)) = (name, score) {
                    players.push((name, score as u32));
                }
            }
        }

        if players.is_empty() {
            None
        } else {
            Some(players)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn set_to_local_storage(_players: &Vec<(String, u32)>) -> Option<()> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    fn set_to_local_storage(players: &Vec<(String, u32)>) -> Option<()> {
        let window = web_sys::window()?;
        let local_storage = window.local_storage().ok()??;

        let array = js_sys::Array::new();
        for player in players {
            let item = js_sys::Array::new();
            item.push(&JsValue::from(&player.0));
            item.push(&JsValue::from(player.1));
            array.push(&JsValue::from(item));
        }
        if let Ok(storage_string) = JSON::stringify(&JsValue::from(array)) {
            let storage_string: String = storage_string.into();
            local_storage
                .set_item("leader_board", &storage_string)
                .unwrap();
        }
        Some(())
    }

    pub fn get_best_score(&self) -> Option<u32> {
        self.players.iter().map(|x| x.1).max()
    }

    pub fn get_lowest_score(&self) -> Option<u32> {
        self.players.iter().map(|x| x.1).min()
    }

    pub fn add_player(&mut self, name: String, score: u32) {
        self.players.push((name, score));
        self.players.sort_by_key(|x| !x.1);
        self.players.truncate(MAX_PLAYERS);
        Self::set_to_local_storage(&self.players);
    }

    pub fn get_best_player(&self) -> Option<String> {
        self.players.iter().max_by_key(|x| x.1).map(|x| x.0.clone())
    }
}

pub fn change_leaders(
    mut leader_board: ResMut<LeaderBoard>,
    game_score: Res<GameScore>,
    mut state: ResMut<NextState<LeaderBoardState>>,
) {
    if let Some(score) = leader_board.get_lowest_score() {
        if game_score.current_score > score {
            leader_board.add_player("new player".into(), game_score.current_score);
        }
    }
    state.set(LeaderBoardState::Show);
}

fn spawn_leader_board(
    mut commands: Commands,
    leader_board: Res<LeaderBoard>,
    asset_server: Res<AssetServer>,
    q_main: Query<Entity, With<MainCenter>>,
) {
    let font = asset_server.load("fonts/ThinPixel7.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 35.0,
        color: Color::DARK_GRAY,
    };

    let main = q_main.get_single().expect("Main not found");

    commands.entity(main).with_children(|main| {
        main.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                align_self: AlignSelf::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            z_index: ZIndex::Global(100),
            ..default()
        })
        .with_children(|parent| {
            for (name, value) in leader_board.players.iter() {
                spawn_leader_line(parent, &text_style, name, value);
            }
        })
        .insert(LeaderBoardNode);
    });
}

fn spawn_leader_line(parent: &mut ChildBuilder, text_style: &TextStyle, text: &str, value: &u32) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(LINE_WIDTH),
                height: Val::Px(LINE_HEIGHT),
                border: UiRect::all(Val::Px(LINE_BORDER)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(LINE_PADDING)),
                ..default()
            },
            border_color: BorderColor(LINE_BORDER_COLOR),
            background_color: LINE_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(text, text_style.clone()));
            parent.spawn(TextBundle::from_section(
                value.to_string(),
                text_style.clone(),
            ));
        });
}

fn despawn_leader_board(
    mut commands: Commands,
    leader_board_query: Query<Entity, With<LeaderBoardNode>>,
) {
    for entity in leader_board_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
