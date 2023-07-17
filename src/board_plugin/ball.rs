use bevy::prelude::*;
use rand::prelude::*;

#[derive(Debug, Component)]
#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
pub struct Ball;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
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

    pub fn get(&self) -> Color {
        match self {
            BallColor::Red => Color::hex("ec1c24").unwrap(),
            BallColor::Blue => Color::hex("0e1bd2").unwrap(),
            BallColor::Cyan => Color::hex("00a8f3").unwrap(),
            BallColor::Green => Color::hex("069a30").unwrap(),
            BallColor::Purple => Color::hex("d71fda").unwrap(),
            BallColor::Brown => Color::hex("b97a56").unwrap(),
            BallColor::Yellow => Color::hex("fff200").unwrap(),
        }
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
