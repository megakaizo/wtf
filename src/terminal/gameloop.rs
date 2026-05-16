use std::io::{stdout};
use crossterm::{            
    cursor::{Hide, MoveTo}, execute, terminal::{
        Clear, 
        ClearType, 
        EnterAlternateScreen, 
        LeaveAlternateScreen, 
        disable_raw_mode, 
        enable_raw_mode,
    },
    event::{EnableMouseCapture, DisableMouseCapture},
};

use crate::terminal::menu::input::run_menu;
use crate::terminal::gameplay::states::GameState;
use crate::terminal::gameplay::start::run_gameplay;


pub fn start_game_session() {
    enable_raw_mode().unwrap();
    
    let mut stdout = stdout();
    let mut state = GameState::Menu;
    
    execute!(
        stdout,
        EnableMouseCapture,
        EnterAlternateScreen,
        Hide,
    ).unwrap(); 
    
    loop {
        execute!(
            stdout, 
            Clear(ClearType::All),
            MoveTo(0, 0),
        ).unwrap();
        match state {
            GameState::Menu => run_menu(&mut state, &mut stdout),
            GameState::End => break,
            GameState::Playing => run_gameplay(&mut stdout),
        }
    }
    execute!(
        stdout,
        DisableMouseCapture,
        LeaveAlternateScreen,
    ).unwrap();
    disable_raw_mode().unwrap();
}
