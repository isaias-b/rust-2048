use position::Position;
use value::Value;

pub mod position;
pub mod value;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub value: Value,
    pub position: Position,
}