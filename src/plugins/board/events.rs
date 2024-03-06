use super::ball::BallColor;
use bevy::prelude::Event;

#[derive(Event)]
pub struct SpawnNewBallEvent(pub BallColor);

#[derive(Event)]
pub struct ChangeNextBallsEvent;
