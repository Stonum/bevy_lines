use bevy::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;

use crate::components::*;
use crate::MIN_BALLS_INLINE;

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

    // get all lines for board. horisontal, vertical and diagonal
    fn get_lines(&self) -> Vec<Vec<Coordinates>> {
        let mut lines: Vec<_> = vec![];
        let count = self.tiles_count as i32;

        // diagonal lines
        for x in -count..count * 2 {
            let mut diagonal = vec![];
            let mut rev_diagonal = vec![];

            for y in 0..count {
                let row = y;
                let col = x + y;

                if row >= 0 && row < count && col >= 0 && col < count {
                    diagonal.push(Coordinates(row as u8, col as u8));
                }

                let row = y;
                let col = x - y;

                if row >= 0 && row < count && col >= 0 && col < count {
                    rev_diagonal.push(Coordinates(row as u8, col as u8));
                }
            }
            if diagonal.len() >= MIN_BALLS_INLINE {
                lines.push(diagonal);
            }
            if rev_diagonal.len() >= MIN_BALLS_INLINE {
                lines.push(rev_diagonal);
            }
        }

        // vertical lines
        for x in 0..self.tiles_count {
            let mut line = vec![];

            for y in 0..self.tiles_count {
                line.push(Coordinates(x, y));
            }

            lines.push(line);
        }

        // horisontal lines
        for y in 0..self.tiles_count {
            let mut line = vec![];

            for x in 0..self.tiles_count {
                line.push(Coordinates(x, y));
            }

            lines.push(line);
        }

        lines
    }

    pub fn get_free_tile(&self) -> Option<Coordinates> {
        let free_tiles: Vec<&Coordinates> = self
            .tiles_map
            .iter()
            .filter(|(_coord, color)| color.is_none())
            .map(|(coord, _color)| coord)
            .collect();

        if let Some(coord) = free_tiles.choose(&mut rand::thread_rng()) {
            return Some(**coord);
        }
        None
    }

    pub fn phisical_pos(&self, coord: &Coordinates) -> Vec2 {
        let offset = -self.phisical_size / 2.;
        Vec2::new(
            (coord.0 as f32 * self.tile_size) + (self.tile_size / 2.) + offset,
            -((coord.1 as f32 * self.tile_size) + (self.tile_size / 2.) + offset),
        )
    }

    pub fn logical_pos(&self, win: &Window, pos: Vec2) -> Option<Coordinates> {
        let window_size = Vec2::new(win.width(), win.height());
        let position = pos - window_size / 2.;
        let size = self.phisical_size / 2.;

        if size < position.x.abs() || size < position.y.abs() {
            return None;
        }

        let coord = Coordinates(
            ((position.x + size) / self.tile_size) as u8,
            ((position.y - size).abs() / self.tile_size) as u8,
        );
        Some(coord)
    }

    pub fn get_balls_for_despawn(&self) -> Vec<Vec<Coordinates>> {
        let lines = self.get_lines();
        let mut result = vec![];
        let mut acc = vec![];

        for line in lines {
            let mut last_color = None;
            acc.clear();

            for coord in line {
                let color = self.tiles_map.get(&coord).unwrap();

                match color {
                    Some(color) => {
                        if last_color.is_some() && last_color != Some(*color) {
                            last_color = None;
                            if acc.len() >= 5 {
                                result.push(acc.clone());
                            }
                            acc.clear();
                        }
                        if last_color.is_none() || last_color == Some(*color) {
                            last_color = Some(*color);
                            acc.push(coord);
                        }
                    }
                    // clear if tile is empty
                    None => {
                        last_color = None;
                        if acc.len() >= 5 {
                            result.push(acc.clone());
                        }
                        acc.clear();
                    }
                }
            }

            // clear on new line
            if acc.len() >= 5 {
                result.push(acc.clone());
            }
        }

        result
    }
}

#[derive(Default, Resource)]
pub struct Game {
    pub score: u32,
}
