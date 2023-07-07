use crate::components::Coordinates;

pub struct SpawnBallsEvent;

pub struct SelectBallEvent(pub Coordinates);

pub struct MoveBallEvent(pub Coordinates);
