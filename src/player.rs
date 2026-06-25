use crate::components::{Player, Position, Viewshed};
use crate::map::{Map, TileType};
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
        None => RunState::Paused,
        Some(key) => match key {
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
            _ => RunState::Paused,
        },
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) -> RunState {
    let players = ecs.read_storage::<Player>();
    let mut positions = ecs.write_storage::<Position>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    let map = ecs.fetch::<Map>();

    for (_player, position, viewshed) in (&players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(position.x + delta_x, position.y + delta_y);
        // since we have walls all around the map, we don't need to check that the destination_idx
        // is within the map boundaries before indexing into the map.
        if map.tiles[destination_idx] != TileType::Wall {
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
    RunState::Running
}
