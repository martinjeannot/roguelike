mod components;
mod map;
mod player;
mod rect;
mod visibility_system;

use crate::components::{Player, Position, Renderable, Viewshed};
use crate::map::Map;
use crate::player::handle_player_input;
use crate::visibility_system::VisibilitySystem;
use bracket_lib::prelude::*;
use specs::prelude::*;

struct State {
    ecs: World,
}

impl State {
    /// Run all registered systems.
    fn run_systems(&mut self) {
        let mut visibility_system = VisibilitySystem {};
        visibility_system.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // clear screen

        // ### 1. INPUTS
        handle_player_input(self, ctx);

        // ### 2. SYSTEMS
        self.run_systems();

        // ### 3. RENDERING
        let map = self.ecs.fetch::<Map>();
        map.draw(ctx);

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

    // ### Game state creation
    let mut game_state = State { ecs: World::new() };

    // ### Map creation
    let map = Map::new(80, 50);
    let (player_x, player_y) = map.rooms[0].center();
    game_state.ecs.insert(map);

    // ### Components registration
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Viewshed>();
    game_state.ecs.register::<Player>();

    // ### Entity initialization

    // player initialization
    game_state
        .ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Player)
        .build();

    main_loop(context, game_state)
}
