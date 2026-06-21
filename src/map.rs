use crate::rect::Rect;
use bracket_lib::color::{BLACK, RGB};
use bracket_lib::prelude::{to_cp437, BTerm, RandomNumberGenerator};
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y * 80 + x) as usize
}

pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80 * 50];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let width = rng.range(MIN_SIZE, MAX_SIZE);
        let height = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 79 - width) - 1;
        let y = rng.roll_dice(1, 49 - height) - 1;
        let new_room = Rect::new(x, y, width, height);
        // if no other room intersects with the new one, we can add it to the map
        if !rooms.iter().any(|room| room.intersect(&new_room)) {
            apply_room_to_map(&new_room, &mut map);
            // connect the new room to the previous room with a tunnel
            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms, map)
}

/// Generate a map with solid boundaries and 400 randomly placed walls.
/// May look erratic... Use for test purpose only.
#[allow(unused)]
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall; // North wall
        map[xy_idx(x, 49)] = TileType::Wall; // South wall
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall; // West wall
        map[xy_idx(79, y)] = TileType::Wall; // East wall
    }

    // randomly generating a bunch of walls
    let mut rng = RandomNumberGenerator::new();
    for _ in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        // (40, 25) is player's starting position
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            let idx = xy_idx(x, y);
            if idx < map.len() {
                map[idx] = TileType::Floor;
            }
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx < map.len() {
            map[idx] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx < map.len() {
            map[idx] = TileType::Floor;
        }
    }
}

pub fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    for (i, tile) in map.iter().enumerate() {
        let x = (i % 80) as i32;
        let y = (i / 80) as i32;
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
