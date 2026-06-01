use std::collections::HashMap;

use serde::Deserialize;

use crate::world::types::{Cell, CellRule};


#[derive(Debug, Deserialize)]
pub struct WorldConfig {
    pub size: MapSize,
    pub factions: FactionsRules,
    pub worldgen: WorldGenRules,
    pub cells: HashMap<Cell, CellRule>,
}

#[derive(Debug, Deserialize)]
pub struct MapSize {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Deserialize)]
pub struct FactionsRules {
    pub energy_per_faction: u16,
    pub total_factions: u16,
}

#[derive(Debug, Deserialize)]
pub struct WorldGenRules {
    pub min_req_base_distance: u16,
    pub water_coverage: f32,
    pub forest_coverage: f32,
    pub mountains_coverage: f32,
}
