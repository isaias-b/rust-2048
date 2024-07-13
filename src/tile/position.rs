use std::collections::HashMap;

use bevy::prelude::Component;

use crate::direction::Direction;

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub type Line = Vec<Position>;
pub type LineTraversals = Vec<Line>;
pub type TraversalMap = HashMap<Direction, LineTraversals>;

impl Position {
    pub fn generate_traversal_map(size: usize) -> TraversalMap {
        let mut mapping = HashMap::new();
        mapping.insert(
            Direction::Left,
            Position::generate_line_traversal(size, false, false),
        );
        mapping.insert(
            Direction::Right,
            Position::generate_line_traversal(size, false, true),
        );
        mapping.insert(
            Direction::Up,
            Position::generate_line_traversal(size, true, false),
        );
        mapping.insert(
            Direction::Down,
            Position::generate_line_traversal(size, true, true),
        );
        mapping
    }

    pub fn move_within(&self, direction: &Direction) -> Position {
        let mut new_pos = *self;
        match direction {
            Direction::Up => {
                new_pos.row += 1;
            }
            Direction::Down => {
                new_pos.row -= 1;
            }
            Direction::Left => {
                new_pos.col -= 1;
            }
            Direction::Right => {
                new_pos.col += 1;
            }
        }
        new_pos
    }


    pub fn generate_line_traversal(size: usize, transpose: bool, mirror: bool) -> LineTraversals {
        let range_eye: Vec<usize> = (0..size).collect();
        let range_inv: Vec<usize> = (0..size).rev().collect();
        let rows = &range_eye;
        let mut traversals = Vec::with_capacity(size);
        let cols = if mirror { &range_inv } else { &range_eye };
        for &row in rows {
            let mut row_indices = Vec::with_capacity(size);
            for &col in cols {
                row_indices.push(if transpose { (col, row) } else { (row, col) });
            }
            let line_traversal = row_indices
                .into_iter()
                .map(|(row, col)| Position { row, col })
                .collect();
            traversals.push(line_traversal);
        }
        traversals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mapping_of(direction: &Direction) -> Vec<Vec<(usize, usize)>> {
        Position::generate_traversal_map(4)
            .get(direction)
            .unwrap()
            .into_iter()
            .map(|line| {
                line.iter()
                    .map(|pos| (pos.row, pos.col))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>()
    }

    #[test]
    fn test_left_index_mapping() {
        let mapping = mapping_of(&Direction::Left);

        assert_eq!(mapping[0], vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
        assert_eq!(mapping[1], vec![(1, 0), (1, 1), (1, 2), (1, 3)]);
        assert_eq!(mapping[2], vec![(2, 0), (2, 1), (2, 2), (2, 3)]);
        assert_eq!(mapping[3], vec![(3, 0), (3, 1), (3, 2), (3, 3)]);
    }

    #[test]
    fn test_right_index_mapping() {
        let mapping = mapping_of(&Direction::Right);

        assert_eq!(mapping[0], vec![(0, 3), (0, 2), (0, 1), (0, 0)]);
        assert_eq!(mapping[1], vec![(1, 3), (1, 2), (1, 1), (1, 0)]);
        assert_eq!(mapping[2], vec![(2, 3), (2, 2), (2, 1), (2, 0)]);
        assert_eq!(mapping[3], vec![(3, 3), (3, 2), (3, 1), (3, 0)]);
    }

    #[test]
    fn test_up_index_mapping() {
        let mapping: Vec<Vec<(usize, usize)>> = mapping_of(&Direction::Up);

        assert_eq!(mapping[0], vec![(0, 0), (1, 0), (2, 0), (3, 0)]);
        assert_eq!(mapping[1], vec![(0, 1), (1, 1), (2, 1), (3, 1)]);
        assert_eq!(mapping[2], vec![(0, 2), (1, 2), (2, 2), (3, 2)]);
        assert_eq!(mapping[3], vec![(0, 3), (1, 3), (2, 3), (3, 3)]);
    }

    #[test]
    fn test_down_index_mapping() {
        let mapping = mapping_of(&Direction::Down);

        assert_eq!(mapping[0], vec![(3, 0), (2, 0), (1, 0), (0, 0)]);
        assert_eq!(mapping[1], vec![(3, 1), (2, 1), (1, 1), (0, 1)]);
        assert_eq!(mapping[2], vec![(3, 2), (2, 2), (1, 2), (0, 2)]);
        assert_eq!(mapping[3], vec![(3, 3), (2, 3), (1, 3), (0, 3)]);
    }
}
