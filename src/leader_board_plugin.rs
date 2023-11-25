use bevy::prelude::*;

use crate::game_score_plugin::GameScore;
use crate::GameState;

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
        let mut players = Vec::with_capacity(10);
        // TODO: load from file
        for i in 1..=10 {
            players.push(("Player ".to_string() + &i.to_string(), i * 100));
        }
        players.sort_by_key(|x| !x.1); // reversed sotring
        Self { players }
    }

    pub fn get_best_score(&self) -> Option<u32> {
        self.players.iter().map(|x| x.1).max()
    }

    pub fn get_lowest_score(&self) -> Option<u32> {
        self.players.iter().map(|x| x.1).min()
    }

    pub fn add_player(&mut self, name: String, score: u32) {
        self.players.sort_by_key(|x| !x.1);
        self.players.pop();
        self.players.push((name, score));
        self.players.sort_by_key(|x| !x.1);
    }
}

pub fn change_leaders(mut leader_board: ResMut<LeaderBoard>, game_score: Res<GameScore>) {
    if let Some(score) = leader_board.get_lowest_score() {
        if game_score.current_score > score {
            leader_board.add_player("new player".into(), game_score.current_score);
        }
    }
}

pub fn spawn_leader_board(mut _commands: Commands, _leader_board: Res<LeaderBoard>) {
    // TODO
}

pub fn despawn_leader_board(mut _commands: Commands) {
    // TODO
}
