use std::io::{Stdout};

use crossterm::{event::{Event, read, KeyCode}};

use crate::terminal::rendering::menu::show_menu;
use crate::terminal::gameplay::states::GameState;


pub fn run_menu(state: &mut GameState, stdout: &mut Stdout) {
    show_menu(stdout);
    if let Event::Key(event) = read().unwrap() {
        match event.code {
            KeyCode::Esc => *state = GameState::End,
            KeyCode::Char(' ') => *state = GameState::Playing,
            _ => {},
        }
    }
}
