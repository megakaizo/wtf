use std::fs;
use std::error::Error;

use serde::Deserialize;

use crate::world::config::WorldConfig;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub world: WorldConfig,
    pub ai: AIConfig,
    pub terminal: TerminalConfig
}


#[derive(Deserialize, Debug)]
pub struct AIConfig {
    pub ai_count: u16
}


#[derive(Deserialize, Debug)]
pub struct TerminalConfig {
    pub offset_x: u16,
    pub offset_y: u16
}


impl Config {
    pub fn from_toml(file: String) -> Result<Self, Box<dyn Error>> {
        let exe = std::env::current_exe()?;
        let dir = exe
            .parent().unwrap()
            .parent().unwrap()
            .parent().unwrap();
        let path = dir.join(file);
        let s = fs::read_to_string(path).expect(dir.to_str().unwrap());
        let config: Config = toml::from_str(&s)?;
        Ok(config)
    }
}
