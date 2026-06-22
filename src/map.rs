use crate::rect::Rect;
use bracket_lib::color::{BLACK, RGB};
use bracket_lib::prelude::{to_cp437, BTerm, RandomNumberGenerator};
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let mut map = Map {
            tiles: vec![TileType::Wall; (width * height) as usize],
            rooms: Vec::new(),
            width,
            height,
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let width = rng.range(MIN_SIZE, MAX_SIZE);
            let height = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, (map.width - 1) - width) - 1;
            let y = rng.roll_dice(1, (map.height - 1) - height) - 1;
            let new_room = Rect::new(x, y, width, height);
            // if no other room intersects with the new one, we can add it to the map
            if !map.rooms.iter().any(|room| room.intersect(&new_room)) {
                map.apply_room_to_map(&new_room);
                // connect the new room to the previous room with a tunnel
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                if idx < self.tiles.len() {
                    self.tiles[idx] = TileType::Floor;
                }
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx < self.tiles.len() {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx < self.tiles.len() {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn draw(&self, ctx: &mut BTerm) {
        for (i, tile) in self.tiles.iter().enumerate() {
            let x = (i % self.width as usize) as i32;
            let y = (i / self.width as usize) as i32;
            match tile {
                TileType::Floor => ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::named(BLACK),
                    to_cp437('.'),
                ),
                TileType::Wall => ctx.set(
                    x,
                    y,
                    RGB::from_f32(0., 1., 0.),
                    RGB::named(BLACK),
                    to_cp437('#'),
                ),
            }
        }
    }
}
