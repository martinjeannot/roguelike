use crate::components::{Monster, Name, Position, Viewshed};
use crate::player::PlayerPosition;
use bracket_lib::prelude::*;
use specs::{Join, ReadExpect, ReadStorage, System};

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, PlayerPosition>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_position, position, viewshed, monster, name) = data;

        for (position, viewshed, name, _) in (&position, &viewshed, &name, &monster).join() {
            if viewshed
                .visible_tiles
                .contains(&Point::new(player_position.x, player_position.y))
            {
                println!(
                    "{} at ({}, {}) sees the player at ({}, {})",
                    name.0, position.x, position.y, player_position.x, player_position.y
                );
            }
        }
    }
}
