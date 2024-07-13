use std::fmt;
use std::str::FromStr;

use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Component)]
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