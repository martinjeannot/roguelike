mod components;
mod map;
mod player;

use crate::components::{Player, Position, Renderable};
use crate::map::{draw_map, new_map, TileType};
use crate::player::handle_player_input;
use bracket_lib::prelude::*;
use specs::prelude::*;

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
