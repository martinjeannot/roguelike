use crate::components::{CombatStats, DamageDealer, Player, Position, Viewshed};
use crate::map::Map;
use crate::{RunState, State};
use bracket_lib::prelude::{BTerm, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub struct PlayerPosition {
    pub x: i32,
    pub y: i32,
}

pub fn handle_player_input(game_state: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => RunState::AwaitingInput,
        Some(key) => match key {
            // Cardinal directions
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, &mut game_state.ecs)
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, &mut game_state.ecs)
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, &mut game_state.ecs)
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, &mut game_state.ecs)
            }
            // Ordinal directions
            VirtualKeyCode::Numpad9 | VirtualKeyCode::Y => {
                try_move_player(1, -1, &mut game_state.ecs)
            }
            VirtualKeyCode::Numpad7 | VirtualKeyCode::U => {
                try_move_player(-1, -1, &mut game_state.ecs)
            }
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => {
                try_move_player(1, 1, &mut game_state.ecs)
            }
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => {
                try_move_player(-1, 1, &mut game_state.ecs)
            }
            _ => RunState::AwaitingInput,
        },
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) -> RunState {
    let entities = ecs.entities();
    let players = ecs.read_storage::<Player>();
    let mut positions = ecs.write_storage::<Position>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    let map = ecs.fetch::<Map>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let mut damage_dealers = ecs.write_storage::<DamageDealer>();

    for (entity, _player, position, viewshed) in
        (&entities, &players, &mut positions, &mut viewsheds).join()
    {
        let destination_idx = map.xy_idx(position.x + delta_x, position.y + delta_y);

        if destination_idx >= (map.width * map.height) as usize {
            // The destination index is out of bounds
            return RunState::PlayerTurn;
        }

        for potential_target in map.tile_contents[destination_idx].iter() {
            let Some(_combat_stats) = combat_stats.get(*potential_target) else {
                continue;
            };
            // register the intent of dealing damage
            damage_dealers
                .insert(
                    entity,
                    DamageDealer {
                        target: *potential_target,
                    },
                )
                .expect("Failed to add damage dealer.");
            // we can't attack AND move in the same tick
            return RunState::PlayerTurn;
        }

        if !map.blocked_tiles[destination_idx] {
            position.x = min(79, max(0, position.x + delta_x));
            position.y = min(49, max(0, position.y + delta_y));
            // player has moved: must recompute viewshed
            viewshed.dirty = true;
            // player has moved: must update player position resource
            let mut player_position = ecs.write_resource::<PlayerPosition>();
            player_position.x = position.x;
            player_position.y = position.y;
        }
    }
    RunState::PlayerTurn
}
