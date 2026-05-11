use std::io::{Stdout, Write};

use crossterm::{queue, cursor::MoveTo, style::Print, event::{Event, read, KeyCode}};

use crate::{generation::init_world, state::GameState};
use crate::render::render_world;


pub fn show_menu(stdout: &mut Stdout) {
    queue!(
        stdout,
        MoveTo(10, 5),
        Print("War. Territory. Fortress."),

        MoveTo(10, 7),
        Print("[SPACE] -> START | [ESC] -> EXIT"),
    ).unwrap();
}


pub fn handle_menu(state: &mut GameState, stdout: &mut Stdout) {
    show_menu(stdout);
    stdout.flush().unwrap();
    if let Event::Key(event) = read().unwrap() {
        match event.code {
            KeyCode::Esc => *state = GameState::End,
            KeyCode::Char(' ') => *state = GameState::Playing,
            _ => {},
        }
    }
}


fn draw_border(width: u16, height: u16, stdout: &mut Stdout) {
    let max_width = width + 1;
    let max_height = height + 1;

    for y in 0..=max_height {
        for x in 0..=max_width {
            let ch = match (x, y) {

                (0, 0) => '+',
                (w, 0) if w == max_width => '+',

                (0, h) if h == max_height => '+',

                (w, h)
                    if w == max_width
                    && h == max_height => '+',

                (0, _) => '|',
                (w, _) if w == max_width => '|',

                (_, 0) => '=',

                (_, h) if h == max_height => '=',

                _ => ' ',
            }; 
            queue!(
                stdout,
                MoveTo(x, y),
                Print(ch)
            ).unwrap();
        }
    }
}

pub fn handle_playing(state: &mut GameState, stdout: &mut Stdout) {
    let width: u16 = 40;
    let height: u16 = 40;
    let forest_cov: f32 = 0.20;
    let water_cov: f32 = 0.15;
    let total_factions = 8;
    let min_req_base_distance = 7;
    let mut total_players = 1;

    let world = init_world(
        &mut total_players, 
        width, 
        height, 
        water_cov,
        forest_cov, 
        total_factions, 
        min_req_base_distance
    );
    draw_border(width, height, stdout);
    render_world(&world, stdout);

    if let Event::Key(event) = read().unwrap() {
        match event.code {
            KeyCode::Esc => *state = GameState::End,
            _ => {},
        }
    }

}

