use bevy::prelude::Event;

#[derive(Event)]
pub struct IncrementCurrentGameScore(pub u32);
