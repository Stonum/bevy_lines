use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Component)]
#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
pub struct Coordinates(pub u8, pub u8);

#[derive(Resource)]
pub struct Board {
    pub entity: Option<Entity>,
    tile_size: f32,
    tiles_count: u8,
    pub tiles_map: HashMap<Coordinates, Option<BallColor>>,
    pub phisical_size: f32,
    pub active_ball: Option<Entity>,
}

impl Board {
    pub fn new(tiles_count: u8, tile_size: f32) -> Self {
        let mut tiles = HashMap::new();
        for x in 0..tiles_count {
            for y in 0..tiles_count {
                tiles.insert(Coordinates(x, y), None);
            }
        }

        Self {
            entity: None,
            tile_size,
            tiles_count,
            phisical_size: tile_size * tiles_count as f32,
            active_ball: None,
            tiles_map: tiles,
        }
    }

    pub fn get_free_tile(&self) -> Option<Coordinates> {
        let mut free_tiles = vec![];
        for (coord, color) in &self.tiles_map {
            match color {
                None => free_tiles.push(coord),
                _ => (),
            }
        }

        if let Some(coord) = free_tiles.choose(&mut rand::thread_rng()) {
            return Some(**coord);
        }
        None
    }

    pub fn phisical_pos(&self, coord: u8) -> f32 {
        let offset = -self.phisical_size / 2.;
        (coord as f32 * self.tile_size) + (self.tile_size / 2.) + offset
    }

    pub fn logical_pos(&self, win: &Window, pos: Vec2) -> Option<Coordinates> {
        let window_size = Vec2::new(win.width(), win.height());
        let position = pos - window_size / 2.;
        let size = self.phisical_size / 2.;

        if size < position.x.abs() || size < position.y.abs() {
            return None;
        }

        let coord = (position + size) / self.tile_size;
        Some(Coordinates(coord.x as u8, coord.y as u8))
    }
}

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
