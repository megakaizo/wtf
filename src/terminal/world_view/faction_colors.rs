use crossterm::style::Color;

use crate::world::types::Faction;


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


impl Faction {
    pub fn color(&self) -> Color {
        let mut color = FACTION_COLORS[self.id as usize % FACTION_COLORS.len()];
        if self.is_dead {
            color = Color::DarkGrey;
        }
        color
    }
}

