use std::usize;

use crossterm::style::Color;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}


pub struct Faction {
    pub id: u16,
    pub color: Color,
    pub is_dead: bool,
    pub is_ai: bool,
}


#[derive(Clone, Copy)]
pub struct Entity {
    pub cell: Cell,
    pub faction_id: Option<u16>, 
}


pub struct World {
    pub width: u16,
    pub height: u16,
    pub forest_cov: f32,
    pub water_cov: f32,
    pub mountains_cov: f32,
    pub factions: Vec<Faction>,
    pub game_map: Vec<Vec<Entity>>,
    pub energy_per_faction: u16,
}

impl World {
    pub fn get(
        &self, coord: Coord,
    ) -> Entity {
        self.game_map
            [coord.y as usize]
            [coord.x as usize]
    }

    pub fn set(
        &mut self,
        coord: Coord,
        cell: Cell,
        faction_id: Option<u16>,
    ) {
        self.game_map
            [coord.y as usize]
            [coord.x as usize]
                = Entity {
                    cell,
                    faction_id,
                };
    }

    pub fn get_color(
        &self, coord: Coord,
    ) -> Color {
        let entity = self.get(coord);
        let mut color = entity.cell.color();
        if let Some(faction_id) = entity.faction_id {
            color = self.factions[faction_id as usize].color;
        };
    color
    }
    pub fn in_bounds(
        &self,
        coord: Coord,
    ) -> bool {

        coord.x < self.width
            && coord.y < self.height
    }

    pub fn kill_faction(
        &mut self,
        faction_id: u16
    ) {
        let faction = &mut self.factions[faction_id as usize];
        faction.is_dead = true;
        faction.color = Color::DarkGrey;
    }
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


#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Base,
    Territory,
    Fortress,
    Forest,
    Water,
    Mountain,
}


impl Cell {
    pub fn glyph(self) -> char {

        match self {

            Self::Empty => ' ',
            Self::Base => '@',
            Self::Territory => '*',
            Self::Fortress => '#',
            Self::Forest => '^',
            Self::Water => '~',
            Self::Mountain => '▲',
        }
    }

    pub fn color(self) -> Color {

        match self {

            Self::Empty => Color::Black,
            Self::Base => Color::White,
            Self::Territory => Color::White,
            Self::Fortress => Color::White,
            Self::Forest => Color::Green,
            Self::Water => Color::Blue,
            Self::Mountain => Color::Grey,
        }
    }

    pub fn cost(self) -> u16 {

        match self {

            Self::Empty => 1,
            Self::Forest => 2,
            Self::Territory => 1,
            Self::Base => 1,
            Self::Fortress => 999,
            Self::Water => 999,
            Self::Mountain => 3,
        }
    }

    pub fn capture_result(self) -> Cell {

        match self {

            Self::Empty => Cell::Territory,
            Self::Forest => Cell::Territory,
            Self::Territory => Cell::Fortress,
            Self::Base => Cell::Fortress,
            Self::Fortress => Cell::Fortress,
            Self::Water => Cell::Water,
            Self::Mountain => Cell::Territory,
        }
    }
}
