use crate::components::{Position, TileBlocking};
use crate::map::Map;
use specs::prelude::*;

pub struct MapIndexingSystem;

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, TileBlocking>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, positions, tile_blocking, entities) = data;
        map.init_blocked_tiles();
        map.clear_tile_contents();

        for (entity, position) in (&entities, &positions).join() {
            let idx = map.xy_idx(position.x, position.y);

            // if the entity is blocking, update the map
            if tile_blocking.get(entity).is_some() {
                map.blocked_tiles[idx] = true;
            }

            // push the entity to the tile contents (entity implements the Copy trait)
            map.tile_contents[idx].push(entity);
        }
    }
}
