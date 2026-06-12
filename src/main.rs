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

#[derive(Component, Debug)]
struct Player;

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();

    for (_player, position) in (&players, &mut positions).join() {
        position.x = min(79, max(0, position.x + delta_x));
        position.y = min(49, max(0, position.y + delta_y));
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

#[derive(Component, Debug)]
struct LeftMover;

struct LeftWalker;

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (left_movers, mut positions): Self::SystemData) {
        for (_left_mover, position) in (&left_movers, &mut positions).join() {
            position.x -= 1;
            if position.x < 0 {
                position.x = 79;
            }
        }
    }
}

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut left_walker = LeftWalker;
        left_walker.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // clear screen

        handle_player_input(self, ctx);
        // run all registered systems
        self.run_systems();

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
    let context = BTermBuilder::simple80x50()
        .with_title("Rogue")
        .build()?;

    // Game state creation
    let mut game_state = State { ecs: World::new() };
    // Components registration
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<LeftMover>();

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
    for i in 0..10 {
        game_state
            .ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: to_cp437('☺'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .with(LeftMover)
            .build();
    }

    main_loop(context, game_state)
}
