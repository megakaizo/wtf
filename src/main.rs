mod state;
mod game;
mod handlers;
mod generation;
mod types;
mod utils;

use crate::game::start_game_session;


fn main() {
    start_game_session();
}


