use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;
use rand::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Component)]
#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
pub struct Coordinates(pub u8, pub u8);

#[derive(Debug, Component)]
pub struct NextBoard;

#[derive(Debug, Component)]
pub struct NextBall;

#[derive(Debug, Component)]
#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
pub struct Ball;

#[derive(Debug, Component, Clone, Copy)]
#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
pub struct BallColor(pub Color);

impl BallColor {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let color = match rng.gen_range(0..7) {
            0 => Color::hex("ec1c24"), // red,
            1 => Color::hex("0e1bd2"), // blue,
            2 => Color::hex("00a8f3"), // light blue,
            3 => Color::hex("069a30"), // green,
            4 => Color::hex("d71fda"), // purple
            5 => Color::hex("b97a56"), // brown
            _ => Color::hex("fff200"), // yellow
        };
        Self(color.unwrap())
    }
}
