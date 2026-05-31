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



