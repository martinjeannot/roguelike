use crate::components::*;
use crate::map::Map;
use crate::player::PlayerPosition;
use crate::RunState;
use bracket_lib::prelude::*;
use specs::prelude::*;

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, RunState>,
        ReadExpect<'a, Entity>, // player entity
        ReadExpect<'a, PlayerPosition>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, DamageDealer>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            run_state,
            player_entity,
            player_position,
            entities,
            mut positions,
            mut damage_dealers,
            mut viewsheds,
            monsters,
        ) = data;

        if *run_state != RunState::MonsterTurn {
            return;
        }

        for (entity, position, viewshed, _) in
            (&entities, &mut positions, &mut viewsheds, &monsters).join()
        {
            // check if the player has been reached
            let distance_to_player = DistanceAlg::Pythagoras.distance2d(
                Point::new(position.x, position.y),
                Point::new(player_position.x, player_position.y),
            );
            // sqrt(2) = 1.4142...
            if distance_to_player < 1.5 {
                damage_dealers
                    .insert(
                        entity,
                        DamageDealer {
                            target: *player_entity,
                        },
                    )
                    .expect("Failed to add damage dealer.");
                // can't attack AND move in the same tick
                continue;
            }
            if viewshed
                .visible_tiles
                .contains(&Point::new(player_position.x, player_position.y))
            {
                // path to the player
                let path = a_star_search(
                    map.xy_idx(position.x, position.y) as i32,
                    map.xy_idx(player_position.x, player_position.y) as i32,
                    &*map,
                );
                if path.success && path.steps.len() > 1 {
                    // since multiple monsters may move the same turn, we can't wait for
                    // the map_indexing_system to update the map, we have to do it as we go
                    let mut idx = map.xy_idx(position.x, position.y);
                    map.blocked_tiles[idx] = false;
                    position.x = path.steps[1] as i32 % map.width;
                    position.y = path.steps[1] as i32 / map.width;
                    idx = map.xy_idx(position.x, position.y);
                    map.blocked_tiles[idx] = true;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
