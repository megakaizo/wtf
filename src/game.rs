use std::io::{stdout};
use crossterm::{            
    cursor::{Hide, MoveTo}, execute, terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode
    }
};

use crate::{handlers::{handle_menu, handle_playing}, state::GameState};



pub fn start_game_session() {
    enable_raw_mode().unwrap();
    
    let mut stdout = stdout();
    let mut state = GameState::Menu;
    
    execute!(
        stdout,
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
            GameState::Menu => handle_menu(&mut state, &mut stdout),
            GameState::End => break,
            GameState::Playing => handle_playing(&mut stdout),
        }
    }
    execute!(
        stdout,
        LeaveAlternateScreen,
    ).unwrap();
    disable_raw_mode().unwrap();
}
