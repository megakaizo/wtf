use std::io::{Stdout};

use crossterm::{event::{Event, read, KeyCode, MouseEventKind, MouseButton}};

use crate::world::types::{World, Coord};

use crate::terminal::rendering::world_map::{render_world, render_faction_view};


fn turn_player(world: &mut World, stdout: &mut Stdout) {
    match read().unwrap() {
        Event::Mouse(event) => {
            match event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    if event.column == 0 || event.row == 0 {
                        return;
                    }
                    let x = event.column - 1;
                    let y = event.row - 1;
                    let coord = Coord { x, y };
                    if world.in_bounds(coord) {
                        world.action(coord);
                        render_faction_view(world, stdout);
                    }
                }
                _ => {}
            }
        }
        Event::Key(event) => {
            match event.code {
                KeyCode::Esc => return,

                _ => {},
            }
        }
        _ => {} 
    }
}

pub fn start_game_cycle(world: &mut World, stdout: &mut Stdout) {
    let factions_len = world.factions.len();

    loop {
        for i in 0..factions_len {
            if world.factions[i].is_dead {
                continue;
            }
            turn_player(world, stdout);
        }
    }
}


