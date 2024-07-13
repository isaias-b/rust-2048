use bevy::prelude::*;
use std::fmt;
use std::str::FromStr;

pub const MAX_TILE_VALUE: u32 = 2048;
const MAX_TILE_INCREMENT: u32 = MAX_TILE_VALUE.ilog2() - 1;
pub const EMPTY_TILE_BG_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Value {
    Empty,
    Number(u32),
}

impl Value {
    pub fn tile_color(&self) -> Color {
        match self {
            Value::Empty => EMPTY_TILE_BG_COLOR.with_a(0.0),
            Value::Number(value) => {
                let gray: f32 = 0.7;
                let t = (value.ilog2() - 1) as f32 / MAX_TILE_INCREMENT as f32;
                let r = lerp(gray, 1.0, t);
                Color::rgb(r, gray, gray)
            }
        }
    }

    pub fn transparency_value(&self) -> f32 {
        match self {
            Value::Empty => 1.0,
            Value::Number(_) => 0.0,
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            Value::Empty => EMPTY_TILE_BG_COLOR.with_a(0.0),
            Value::Number(value) => Color::BLACK,
        }
    }

    pub fn text_value(&self) -> String {
        match self {
            Value::Empty => "".to_string(),
            Value::Number(value) => value.to_string(),
        }
    }

    pub fn merge(self, other: Value) -> Value {
        match self {
            Value::Empty => other,
            Value::Number(n) => {
                if let Value::Number(m) = other {
                    Value::Number(n + m)
                } else {
                    self
                }
            }
        }
    }

    pub fn to_exponent(&self) -> u32 {
        match self {
            Value::Empty => 0,
            Value::Number(n) => n.trailing_zeros(),
        }
    }
}

impl fmt::Display for Value {
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

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Value::Empty),
            "1" => Ok(Value::Number(2)),
            "2" => Ok(Value::Number(4)),
            "3" => Ok(Value::Number(8)),
            "4" => Ok(Value::Number(16)),
            "5" => Ok(Value::Number(32)),
            "6" => Ok(Value::Number(64)),
            "7" => Ok(Value::Number(128)),
            "8" => Ok(Value::Number(256)),
            "9" => Ok(Value::Number(512)),
            "A" => Ok(Value::Number(1024)),
            "B" => Ok(Value::Number(2048)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_exponent_conversion() {
        let tile = Value::Number(512);
        assert_eq!(tile.to_exponent(), 9);
    }

    #[test]
    fn test_tile_display() {
        let tile = Value::Number(2048);
        assert_eq!(tile.to_string(), "B");
    }

    #[test]
    fn test_tile_from_str() {
        let tile: Value = "B".parse().unwrap();
        assert_eq!(tile, Value::Number(2048));
    }
}
