mod components;
mod damage_system;
mod map;
mod map_indexing_system;
mod melee_combat_system;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

use crate::components::*;
use crate::map::Map;
use crate::monster_ai_system::MonsterAI;
use crate::player::{handle_player_input, PlayerPosition};
use crate::visibility_system::VisibilitySystem;
use bracket_lib::prelude::*;
use specs::prelude::*;

struct State {
    ecs: World,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
}

impl State {
    /// Run all registered systems.
    ///
    /// Order matters here, for example the MapIndexingSystem must run after the MonsterAI system,
    /// because the latter may move monsters around the map, and the former needs to update the
    /// map's blocked tiles and tile contents accordingly.
    fn run_systems(&mut self) {
        let mut visibility_system = VisibilitySystem;
        visibility_system.run_now(&self.ecs);

        let mut monster_ai = MonsterAI;
        monster_ai.run_now(&self.ecs);

        let mut map_indexing_system = map_indexing_system::MapIndexingSystem;
        map_indexing_system.run_now(&self.ecs);

        let mut melee_combat_system = melee_combat_system::MeleeCombatSystem;
        melee_combat_system.run_now(&self.ecs);

        let mut damage_system = damage_system::DamageSystem;
        damage_system.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // clear screen

        let local_run_state;
        {
            let run_state = self.ecs.fetch::<RunState>();
            local_run_state = *run_state;
        }

        let next_run_state = match local_run_state {
            RunState::PreRun => {
                self.run_systems();
                RunState::AwaitingInput
            }
            RunState::AwaitingInput => handle_player_input(self, ctx),
            RunState::PlayerTurn => {
                self.run_systems();
                RunState::MonsterTurn
            }
            RunState::MonsterTurn => {
                self.run_systems();
                RunState::AwaitingInput
            }
        };
        // update the RunState resource
        {
            let mut run_state_writer = self.ecs.write_resource::<RunState>();
            *run_state_writer = next_run_state;
        }

        damage_system::remove_corpses(&mut self.ecs);

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
    let mut game_state = State { ecs: World::new() };

    // ### Map creation
    let map = Map::new(80, 50);
    let room_centers: Vec<(i32, i32)> = map.rooms.iter().map(|room| room.center()).collect();
    game_state.ecs.insert(map);

    // ### Components registration
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Viewshed>();
    game_state.ecs.register::<Chasing>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<Monster>();
    game_state.ecs.register::<Name>();
    game_state.ecs.register::<TileBlocking>();
    game_state.ecs.register::<CombatStats>();
    game_state.ecs.register::<DamageDealer>();
    game_state.ecs.register::<DamageTaker>();

    // ### Entity initialization

    game_state.ecs.insert(RunState::PreRun);
    let mut rng = RandomNumberGenerator::new();

    // player initialization
    let player_entity = game_state
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
        .with(Name("Player".to_string()))
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .build();
    game_state.ecs.insert(player_entity);
    // additionally, we add the player position as a resource
    game_state.ecs.insert(PlayerPosition {
        x: room_centers[0].0,
        y: room_centers[0].1,
    });

    // monsters initialization
    for (i, room_center) in room_centers.iter().skip(1).enumerate() {
        let (glyph, name) = match rng.roll_dice(1, 2) {
            1 => (to_cp437('g'), format!("Goblin #{i}")),
            _ => (to_cp437('o'), format!("Orc #{i}")),
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
            .with(Chasing { target: None })
            .with(Monster)
            .with(Name(name.to_owned()))
            .with(TileBlocking)
            .with(CombatStats {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 4,
            })
            .build();
    }

    main_loop(context, game_state)
}
