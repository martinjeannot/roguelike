use crate::components::{Player, Position, Viewshed};
use crate::map::Map;
use bracket_lib::prelude::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem;

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, position, mut viewshed, player) = data;

        for (entity, position, viewshed) in (&entities, &position, &mut viewshed).join() {
            if !viewshed.dirty {
                continue;
            }
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles =
                field_of_view(Point::new(position.x, position.y), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|point| {
                point.x >= 0 && point.x < map.width && point.y >= 0 && point.y < map.height
            });
            viewshed.dirty = false;

            if let Some(_) = player.get(entity) {
                map.visible_tiles.iter_mut().for_each(|tile| *tile = false);
                for visible_tile in viewshed.visible_tiles.iter() {
                    let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                    map.revealed_tiles[idx] = true;
                    map.visible_tiles[idx] = true;
                }
            }
        }
    }
}
