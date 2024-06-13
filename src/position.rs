use bevy::prelude::*;

#[derive(Component)]
struct Position {
  row: usize,
  col: usize,
}