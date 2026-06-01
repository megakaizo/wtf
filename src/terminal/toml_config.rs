use serde::Deserialize;

use crate::world::config::WorldConfig;


#[derive(Deserialize, Debug)]
pub struct Config {
    world: WorldConfig,
    ai: AIConfig,
    terminal: TerminalConfig
}


#[derive(Deserialize, Debug)]
pub struct AIConfig {
    ai_count: u16
}


#[derive(Deserialize, Debug)]
pub struct TerminalConfig {
    offset_x: u16,
    offset_y: u16
}
