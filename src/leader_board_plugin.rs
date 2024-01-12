use bevy::prelude::*;
use js_sys::JSON;
use wasm_bindgen::prelude::*;

use crate::game_score_plugin::GameScore;
use crate::GameState;

const MAX_PLAYERS: usize = 10;

pub struct LeaderBoardPlugin;

impl Plugin for LeaderBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LeaderBoard::new())
            .add_systems(
                OnEnter(GameState::GameOver),
                (change_leaders, spawn_leader_board),
            )
            .add_systems(OnExit(GameState::GameOver), despawn_leader_board);
    }
}

#[derive(Resource, Debug)]
pub struct LeaderBoard {
    pub players: Vec<(String, u32)>,
}

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
        Some(players)
    }

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
}

pub fn change_leaders(mut leader_board: ResMut<LeaderBoard>, game_score: Res<GameScore>) {
    if let Some(score) = leader_board.get_lowest_score() {
        if game_score.current_score > score {
            leader_board.add_player("new player".into(), game_score.current_score);
        }
    }
}

pub fn spawn_leader_board(
    mut _commands: Commands,
    _leader_board: Res<LeaderBoard>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::Playing);
}

pub fn despawn_leader_board(mut _commands: Commands) {
    // TODO
}
