use std::fs;
use std::error::Error;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::world::config::WorldConfig;


#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub world: WorldConfig,
    pub ai: AIConfig,
    pub terminal: TerminalConfig
}


#[derive(Deserialize, Serialize, Debug)]
pub struct AIConfig {
    pub ai_count: u16
}


#[derive(Deserialize, Serialize, Debug)]
pub struct TerminalConfig {
    pub offset_x: u16,
    pub offset_y: u16
}


impl Config {
    fn config_dir() -> Result<PathBuf, Box<dyn Error>> {
        let exe = std::env::current_exe()?;
        let dir = exe
            .parent().unwrap()
            .parent().unwrap()
            .parent().unwrap();
        Ok(dir.to_path_buf())
    }

    pub fn from_toml(file: String) -> Result<Self, Box<dyn Error>> {
        let dir = Self::config_dir()?;
        let path = dir.join(file);
        let s = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&s)?;
        Ok(config)
    }

    pub fn default() -> Self {
        Config {
            world: WorldConfig::default(),
            ai: AIConfig { ai_count: 3 },
            terminal: TerminalConfig { offset_x: 20, offset_y: 5 },
        }
    }

    pub fn load(file: String) -> Self {
        match Self::from_toml(file) {
            Ok(config) => config,
            Err(_) => {
                let config = Config::default();
                Self::save(&config, "wtf.toml".to_string());
                return config;
            }
        }
    }

    pub fn save(config: &Config, file: String) -> Result<(), Box<dyn Error>> {
        let contents = toml::to_string(config)?;
        let dir = Self::config_dir()?;
        let path = dir.join(file);
        fs::write(path, contents)?;
        Ok(())
    }
}
