pub mod world;
pub mod rendering;
pub mod engine;
pub mod gameplay;
pub mod menu;

use crate::engine::gameloop::start_game_session;


fn main() {
    start_game_session();
}


