use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::world::types::{Cell, CellRule};


#[derive(Debug, Deserialize, Serialize)]
pub struct WorldConfig {
    pub size: MapSize,
    pub factions: FactionsRules,
    pub worldgen: WorldGenRules,
    pub cells: HashMap<Cell, CellRule>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MapSize {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FactionsRules {
    pub energy_per_faction: u16,
    pub total_factions: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorldGenRules {
    pub min_req_base_distance: u16,
    pub water_coverage: f32,
    pub forest_coverage: f32,
    pub mountains_coverage: f32,
}



impl WorldConfig {
    pub fn default() -> Self {
        let size = MapSize {
            width: 20,
            height: 40,
        };
        let factions = FactionsRules {
            energy_per_faction: 5,
            total_factions: 4,
        };
        let worldgen = WorldGenRules {
            min_req_base_distance: 7,
            water_coverage: 0.15,
            forest_coverage: 0.20,
            mountains_coverage: 0.05,
        };

        let mut cells = HashMap::new();
        cells.insert(Cell::Empty,     CellRule { cost: 1, vision: 0,   capture_result: Cell::Territory });
        cells.insert(Cell::Base,      CellRule { cost: 1, vision: 5,   capture_result: Cell::Fortress });
        cells.insert(Cell::Territory, CellRule { cost: 1, vision: 5,   capture_result: Cell::Fortress });
        cells.insert(Cell::Fortress,  CellRule { cost: 999, vision: 5, capture_result: Cell::Fortress });
        cells.insert(Cell::Forest,    CellRule { cost: 2, vision: 0,   capture_result: Cell::Territory });
        cells.insert(Cell::Water,     CellRule { cost: 4, vision: 0,   capture_result: Cell::Bridge });
        cells.insert(Cell::Mountain,  CellRule { cost: 3, vision: 0,   capture_result: Cell::Territory });
        cells.insert(Cell::Bridge,    CellRule { cost: 1, vision: 5,   capture_result: Cell::Bridge });

        WorldConfig {
            size,
            factions,
            worldgen,
            cells,
        }
    }
}

