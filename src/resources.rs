use bevy::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;

use crate::components::*;

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
