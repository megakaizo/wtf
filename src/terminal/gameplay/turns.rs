use std::{io::Stdout, time::Duration};

use crossterm::{event::{Event, poll, read, KeyCode, MouseEventKind, MouseButton}};

use crate::world::types::{World, Coord};
use crate::terminal::rendering::world_map::{render_faction_view};


fn clear_buffer() {
    while poll(Duration::from_millis(0)).unwrap() {
        let _ = read();
    }
}


pub fn turn_player(world: &mut World, stdout: &mut Stdout, offset_x: u16, offset_y: u16) {
    clear_buffer();

    let view_faction_id = world.current_move_faction_id;
    render_faction_view(world, view_faction_id, stdout, offset_x, offset_y);

    match read().unwrap() {
        Event::Mouse(event) => {
            match event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    if event.column <= offset_x || event.row <= offset_y {
                        return;
                    }
                    let x = event.column - offset_x - 1;
                    let y = event.row - offset_y - 1;
                    let coord = Coord { x, y };
                    if world.in_bounds(coord) {
                        world.action(coord);
                        render_faction_view(world, view_faction_id, stdout, offset_x, offset_y);
                    }
                }
                MouseEventKind::Down(MouseButton::Right) => {
                    world.turn_next_faction();
                    render_faction_view(world, view_faction_id, stdout, offset_x, offset_y);
                },
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
