use crossterm::style::Color;

#[derive(Clone, Copy)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

pub type CellType = char;

pub const EMPTY: CellType = ' ';
pub const BASE: CellType = '@';
pub const TERRITORY: CellType = '*';
pub const FORTRESS: CellType = '#';
pub const FOREST: CellType = '^';
pub const WATER: CellType = '~';


pub struct Faction {
    pub id: u16,
    pub color: Color,
    pub is_dead: bool,
    pub is_ai: bool,
}

#[derive(Clone, Copy)]
pub struct Entity {
    pub cell_type: CellType,
    pub faction_id: Option<u16>, 
}

pub struct World {
    pub width: u16,
    pub height: u16,
    pub forest_cov: f32,
    pub water_cov: f32,
    pub factions: Vec<Faction>,
    pub game_map: Vec<Vec<Entity>>,
}


pub const FACTION_COLORS: [Color; 16] = [
    Color::AnsiValue(196), // 0  bright red
    Color::AnsiValue(33),  // 1  bright blue
    Color::AnsiValue(46),  // 2  bright green
    Color::AnsiValue(226), // 3  yellow
    Color::AnsiValue(201), // 4  magenta
    Color::AnsiValue(51),  // 5  cyan

    Color::AnsiValue(208), // 6  orange
    Color::AnsiValue(129), // 7  purple
    Color::AnsiValue(154), // 8  lime
    Color::AnsiValue(39),  // 9  sky blue

    Color::AnsiValue(202), // 10 deep orange
    Color::AnsiValue(93),  // 11 dark violet
    Color::AnsiValue(82),  // 12 toxic green
    Color::AnsiValue(45),  // 13 aqua
    Color::AnsiValue(220), // 14 gold
    Color::AnsiValue(198), // 15 pink
];
