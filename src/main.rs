use bracket_lib::prelude::*;
use specs::prelude::*;
use specs::Component;
use std::cmp::{max, min};

#[derive(Component, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y * 80 + x) as usize
}

fn new_map() -> Vec<TileType> {
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

fn draw_map(map: &[TileType], ctx: &mut BTerm) {
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

#[derive(Component, Debug)]
struct Player;

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, position) in (&players, &mut positions).join() {
        let destination_idx = xy_idx(position.x + delta_x, position.y + delta_y);
        // since we have walls all around the map, we don't need to check that the destination_idx
        // is within the map boundaries before indexing into the map.
        if map[destination_idx] != TileType::Wall {
            position.x = min(79, max(0, position.x + delta_x));
            position.y = min(49, max(0, position.y + delta_y));
        }
    }
}

fn handle_player_input(game_state: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut game_state.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut game_state.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut game_state.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut game_state.ecs),
            _ => {}
        },
    }
}

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // clear screen

        // ### 1. INPUTS
        handle_player_input(self, ctx);

        // ### 2. SYSTEMS
        // run all registered systems
        self.run_systems();

        // ### 3. RENDERING
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (position, renderable) in (&positions, &renderables).join() {
            ctx.set(
                position.x,
                position.y,
                renderable.fg,
                renderable.bg,
                renderable.glyph,
            );
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Rogue").build()?;

    // Game state creation
    let mut game_state = State { ecs: World::new() };

    // Map creation
    game_state.ecs.insert(new_map());

    // Components registration
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Player>();

    // Entity creation
    game_state
        .ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player)
        .build();

    main_loop(context, game_state)
}
