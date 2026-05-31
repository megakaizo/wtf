use std::collections::HashMap;

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct CellRule {
    pub cost: u16,
    pub vision: i32,
    pub capture_result: Cell,
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}


pub struct Faction {
    pub id: u16,
    pub lands: HashMap<Coord, Entity>,
    pub is_dead: bool,
    pub current_move_energy: u16,
}


#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Entity {
    pub cell: Cell,
    pub faction_id: Option<u16>, 
}


pub struct World {
    pub width: u16,
    pub height: u16,
    pub factions: Vec<Faction>,
    pub map: Vec<Entity>,
    pub energy_per_faction: u16,
    pub current_move_faction_id: u16, 
    pub cell_rules: HashMap<Cell, CellRule>,
}


#[derive(
    Clone, 
    Copy, 
    PartialEq, 
    Eq, 
    Hash, 
    Debug, 
    Deserialize
)]
pub enum Cell {
    Empty,
    Base,
    Territory,
    Fortress,
    Forest,
    Water,
    Mountain,
    Bridge,
}


impl Cell {

    pub fn cost(self) -> u16 {

        match self {

            Self::Empty => 1,
            Self::Forest => 2,
            Self::Territory => 1,
            Self::Base => 1,
            Self::Fortress => 999,
            Self::Water => 4,
            Self::Mountain => 3,
            Self::Bridge => 1,
        }
    }

    pub fn capture_result(self) -> Cell {

        match self {

            Self::Empty => Cell::Territory,
            Self::Forest => Cell::Territory,
            Self::Territory => Cell::Fortress,
            Self::Base => Cell::Fortress,
            Self::Fortress => Cell::Fortress,
            Self::Water => Cell::Bridge,
            Self::Mountain => Cell::Territory,
            Self::Bridge => Cell::Bridge,
        }
    }

    pub fn vision(self) -> i32 {
        match self {
            Self::Empty => 0,
            Self::Forest => 0,
            Self::Territory => 5,
            Self::Base => 5,
            Self::Fortress => 5,
            Self::Water => 0,
            Self::Mountain => 0,
            Self::Bridge => 5,

        }
    }
}
