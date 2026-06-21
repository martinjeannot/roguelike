use bracket_lib::color::{BLACK, RGB};
use bracket_lib::prelude::{to_cp437, BTerm, RandomNumberGenerator};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y * 80 + x) as usize
}

pub fn new_map() -> Vec<TileType> {
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
