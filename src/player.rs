use crate::components::{Player, Position};
use crate::map::{xy_idx, TileType};
use crate::State;
use bracket_lib::prelude::{BTerm, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn handle_player_input(game_state: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
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
            _ => {}
        },
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, position) in (&players, &mut positions).join() {
        let destination_idx = xy_idx(position.x + delta_x, position.y + delta_y);
        // since we have walls all around the map, we don't need to check that the destination_idx
        // is within the map boundaries before indexing into the map.
        if map[destination_idx] != TileType::Wall {
            position.x = min(79, max(0, position.x + delta_x));
            position.y = min(49, max(0, position.y + delta_y));
        }
    }
}
