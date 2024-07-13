use bevy::prelude::{Component, Event};

use crate::tile::{position::Position, value::Value, Tile};

#[derive(Event, Debug, Clone, Component)]
pub enum Action {
    SpawnRandomTile(Tile),
    SlideTile(Tile, Position),
    MergeTiles(Tile, Tile, Position, Value),
}
