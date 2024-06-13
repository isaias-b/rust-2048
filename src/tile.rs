use bevy::prelude::*;
use std::fmt;
use std::str::FromStr;

pub const MAX_TILE_VALUE: u32 = 2048;
const MAX_TILE_INCREMENT: u32 = MAX_TILE_VALUE.ilog2() - 1;
const EMPTY_TILE_BG_COLOR: Color = Color::rgb(0.6, 0.6, 0.6);

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Number(u32),
}

impl Tile {
    pub fn color(&self) -> Color {
        match self {
            Tile::Empty => EMPTY_TILE_BG_COLOR,
            Tile::Number(value) => {
                let gray: f32 = 0.8;
                let t = (value.ilog2() - 1) as f32 / MAX_TILE_INCREMENT as f32;
                let r = lerp(gray, 1.0, t);
                Color::rgb(r, gray, gray)
            }
        }
    }

    pub fn to_exponent(&self) -> u32 {
        match self {
            Tile::Empty => 0,
            Tile::Number(n) => n.trailing_zeros(),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_exponent() {
            0 => write!(f, "0"),
            1 => write!(f, "1"),
            2 => write!(f, "2"),
            3 => write!(f, "3"),
            4 => write!(f, "4"),
            5 => write!(f, "5"),
            6 => write!(f, "6"),
            7 => write!(f, "7"),
            8 => write!(f, "8"),
            9 => write!(f, "9"),
            10 => write!(f, "A"),
            11 => write!(f, "B"),
            _ => write!(f, "0"), // Handle unexpected values gracefully
        }
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Tile::Empty),
            "1" => Ok(Tile::Number(2)),
            "2" => Ok(Tile::Number(4)),
            "3" => Ok(Tile::Number(8)),
            "4" => Ok(Tile::Number(16)),
            "5" => Ok(Tile::Number(32)),
            "6" => Ok(Tile::Number(64)),
            "7" => Ok(Tile::Number(128)),
            "8" => Ok(Tile::Number(256)),
            "9" => Ok(Tile::Number(512)),
            "A" => Ok(Tile::Number(1024)),
            "B" => Ok(Tile::Number(2048)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
            Direction::Up => write!(f, "U"),
            Direction::Down => write!(f, "D"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_exponent_conversion() {
        let tile = Tile::Number(512);
        assert_eq!(tile.to_exponent(), 9);
    }

    #[test]
    fn test_tile_display() {
        let tile = Tile::Number(2048);
        assert_eq!(tile.to_string(), "B");
    }

    #[test]
    fn test_tile_from_str() {
        let tile: Tile = "B".parse().unwrap();
        assert_eq!(tile, Tile::Number(2048));
    }
}
