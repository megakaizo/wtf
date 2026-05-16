use crossterm::style::{Color, Attribute};

use crate::world::types::Cell;


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
            Self::Bridge => '=',
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
            Self::Bridge => Color::White,
        }
    }

    pub fn attribute(self) -> Attribute {
        match self {
            Self::Empty => Attribute::NormalIntensity,
            Self::Forest => Attribute::NormalIntensity,
            Self::Territory => Attribute::NormalIntensity,
            Self::Base => Attribute::Bold,
            Self::Fortress => Attribute::NormalIntensity,
            Self::Water => Attribute::NormalIntensity,
            Self::Mountain => Attribute::NormalIntensity,
            Self::Bridge => Attribute::Bold,

        }
    }
}
