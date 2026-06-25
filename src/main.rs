mod components;
mod map;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

use crate::components::{Monster, Name, Player, Position, Renderable, Viewshed};
use crate::map::Map;
use crate::monster_ai_system::MonsterAI;
use crate::player::{handle_player_input, PlayerPosition};
use crate::visibility_system::VisibilitySystem;
use bracket_lib::prelude::*;
use specs::prelude::*;

struct State {
    ecs: World,
    run_state: RunState,
}

enum RunState {
    Paused,
    Running,
}

impl State {
    /// Run all registered systems.
    fn run_systems(&mut self) {
        let mut visibility_system = VisibilitySystem {};
        visibility_system.run_now(&self.ecs);

        let mut monster_ai = MonsterAI {};
        monster_ai.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // clear screen

        self.run_state = match self.run_state {
            RunState::Paused => handle_player_input(self, ctx),
            RunState::Running => {
                self.run_systems();
                RunState::Paused
            }
        };

        // ### RENDERING
        let map = self.ecs.fetch::<Map>();
        map.draw(ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (position, renderable) in (&positions, &renderables).join() {
            let idx = map.xy_idx(position.x, position.y);
            if map.visible_tiles[idx] {
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
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Rogue").build()?;

    // ### Game state creation
    let mut game_state = State {
        ecs: World::new(),
        run_state: RunState::Running,
    };

    // ### Map creation
    let map = Map::new(80, 50);
    let room_centers: Vec<(i32, i32)> = map.rooms.iter().map(|room| room.center()).collect();
    game_state.ecs.insert(map);

    // ### Components registration
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Viewshed>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<Monster>();
    game_state.ecs.register::<Name>();

    // ### Entity initialization

    let mut rng = RandomNumberGenerator::new();

    // player initialization
    game_state
        .ecs
        .create_entity()
        .with(Position {
            x: room_centers[0].0,
            y: room_centers[0].1,
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
    // additionally, we add the player position as a resource
    game_state.ecs.insert(PlayerPosition {
        x: room_centers[0].0,
        y: room_centers[0].1,
    });

    // monsters initialization
    for room_center in room_centers.iter().skip(1) {
        let (glyph, name) = match rng.roll_dice(1, 2) {
            1 => (to_cp437('g'), "Goblin"),
            _ => (to_cp437('o'), "Orc"),
        };

        game_state
            .ecs
            .create_entity()
            .with(Position {
                x: room_center.0,
                y: room_center.1,
            })
            .with(Renderable {
                glyph,
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster)
            .with(Name(name.to_owned()))
            .build();
    }

    main_loop(context, game_state)
}
