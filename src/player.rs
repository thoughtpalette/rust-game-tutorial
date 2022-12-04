use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use super::{Position, Player, TileType, xy_idx, State};
use std::cmp::{min, max};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79 , max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

// This function takes the current game state and context,
// looks at the key variable in the context, and calls the appropriate
// move command if the relevant movement key is pressed.
pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // match "unwraps" value to get data out of Option
    match ctx.key {
        // Option types have two possible value: None (no data),
        // or Some(x) - indicating that there is data here, held inside.
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left | 
            VirtualKeyCode::Numpad4 | 
            VirtualKeyCode::A | 
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right | 
            VirtualKeyCode::Numpad6 | 
            VirtualKeyCode::D | 
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up | 
            VirtualKeyCode::Numpad8 | 
            VirtualKeyCode::W | 
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down | 
            VirtualKeyCode:: Numpad5 | 
            VirtualKeyCode:: S | 
            VirtualKeyCode:: J => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}