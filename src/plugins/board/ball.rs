use bevy::prelude::*;
use rand::prelude::*;

#[derive(Debug, Component)]
pub struct Ball;

#[derive(Debug, Component)]
pub struct BallEntity {
    pub color: BallColor,
    pub entity: Entity,
}

impl BallEntity {
    pub fn new(color: BallColor, entity: Entity) -> Self {
        Self { color, entity }
    }
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum BallColor {
    Red,
    Blue,
    Cyan,
    Green,
    Purple,
    Brown,
    Yellow,
}

impl BallColor {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..7) {
            0 => BallColor::Red,
            1 => BallColor::Blue,
            2 => BallColor::Cyan,
            3 => BallColor::Green,
            4 => BallColor::Purple,
            5 => BallColor::Brown,
            _ => BallColor::Yellow,
        }
    }
}

impl From<BallColor> for Color {
    fn from(color: BallColor) -> Self {
        let hex = match color {
            BallColor::Red => "ec1c24",
            BallColor::Blue => "0e1bd2",
            BallColor::Cyan => "00a8f3",
            BallColor::Green => "069a30",
            BallColor::Purple => "d71fda",
            BallColor::Brown => "b97a56",
            BallColor::Yellow => "fff200",
        };
        Color::hex(hex).unwrap()
    }
}

#[derive(Resource)]
pub struct BallAssets {
    pub texture: Handle<Image>,
}

impl FromWorld for BallAssets {
    fn from_world(world: &mut World) -> Self {
        // You have full access to anything in the ECS from here.
        // For instance, you can mutate other resources:
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        BallAssets {
            texture: asset_server.load("ball.png"),
        }
    }
}

#[derive(Component, Default)]
pub enum BallAnimationState {
    #[default]
    Up,
    Down,
}

#[derive(Component, Deref, DerefMut)]
pub struct BallAnimationTimer(pub Timer);

impl Default for BallAnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.19, TimerMode::Repeating))
    }
}
