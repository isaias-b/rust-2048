use rand::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::tile::{Direction, Tile, MAX_TILE_VALUE};

#[derive(Debug, Clone)]
pub struct Board {
    pub size: usize,
    pub tiles: Vec<Tile>,
    pub index_map: HashMap<Direction, Vec<Vec<(usize, usize)>>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let mut board = Self {
            size,
            tiles: vec![Tile::Empty; size * size],
            index_map: HashMap::new(),
        };

        board.index_map.insert(
            Direction::Left,
            board.generate_indices(false, false),
        );
        board.index_map.insert(
            Direction::Right,
            board.generate_indices(false, true),
        );
        board.index_map.insert(
            Direction::Up,
            board.generate_indices(true, false),
        );
        board.index_map.insert(
            Direction::Down,
            board.generate_indices(true, true),
        );

        board
    }

    pub fn index(&self, row: usize, col: usize) -> usize {
        row * self.size + col
    }

    pub fn index2(&self, pair: (usize, usize)) -> usize {
        let (row, col) = pair;
        self.index(row, col)
    }

    pub fn get(&self, row: usize, col: usize) -> Tile {
        self.tiles[self.index(row, col)]
    }

    pub fn get2(&self, pair: (usize, usize)) -> Tile {
        let (row, col) = pair;
        self.get(row, col)
    }

    pub fn set(&mut self, row: usize, col: usize, tile: Tile) {
        let idx = self.index(row, col);
        self.tiles[idx] = tile;
    }

    pub fn spawn_random_tile(&mut self) {
        let mut empty_positions = vec![];

        for i in 0..self.size {
            for j in 0..self.size {
                if self.get(i, j) == Tile::Empty {
                    empty_positions.push((i, j));
                }
            }
        }

        if !empty_positions.is_empty() {
            let (x, y) = empty_positions[rand::thread_rng().gen_range(0..empty_positions.len())];
            self.set(
                x,
                y,
                Tile::Number(if rand::thread_rng().gen_bool(0.9) {
                    2
                } else {
                    4
                }),
            );
        }
    }

    pub fn slide_and_merge(&mut self, direction: Direction) -> bool {
        let indices = self.index_map.get(&direction).unwrap();
        let mut new_tiles = self.tiles.clone();
        let mut moved = false;

        for row_indices in indices {
            let mut focus_idx = 0;
            let mut prev = None;

            for &current_cell in row_indices {
                let focus_cell = row_indices[focus_idx];
                let can_slide = current_cell != focus_cell; // means there were empty tiles up to this point

                if let Tile::Number(current_value) = self.get2(current_cell) {
                    if let Some((prev_idx, prev_value)) = prev {
                        let can_merge = prev_value == current_value && prev_value < MAX_TILE_VALUE;
                        let prev_cell = row_indices[prev_idx];

                        if can_merge {
                            new_tiles[self.index2(current_cell)] = Tile::Empty;
                            new_tiles[self.index2(prev_cell)] = Tile::Number(prev_value * 2);
                            prev = None;
                            moved = true;
                        } else {
                            if can_slide {
                                new_tiles[self.index2(current_cell)] = Tile::Empty;
                                new_tiles[self.index2(focus_cell)] = Tile::Number(current_value);
                                moved = true;
                            }
                            prev = Some((focus_idx, current_value));
                            focus_idx += 1;
                        }
                    } else {
                        if can_slide {
                            new_tiles[self.index2(current_cell)] = Tile::Empty;
                            new_tiles[self.index2(focus_cell)] = Tile::Number(current_value);
                            moved = true;
                        }
                        prev = Some((focus_idx, current_value));
                        focus_idx += 1;
                    }
                }
            }
        }

        if moved {
            self.tiles = new_tiles;
        }

        moved
    }
    
    pub fn generate_indices(&self, transpose: bool, mirror: bool) -> Vec<Vec<(usize, usize)>> {
        let range_eye: Vec<usize> = (0..self.size).collect();
        let range_inv: Vec<usize> = (0..self.size).rev().collect();
        let rows = &range_eye;
        let mut indices = Vec::with_capacity(self.size);
        let cols = if mirror { &range_inv } else { &range_eye };
        for &row in rows {
            let mut row_indices = Vec::with_capacity(self.size);
            for &col in cols {
                row_indices.push(if transpose { (col, row) } else { (row, col) });
            }
            indices.push(row_indices);   
        }
        indices
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.size {
            for col in 0..self.size {
                write!(f, "{}", self.get(row, col))?;
            }
        }
        Ok(())
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 16 {
            return Err(());
        }

        let mut board = Board::new(4);
        for (i, hex_char) in s.chars().enumerate() {
            let row = i / 4;
            let col = i % 4;
            let idx = board.index(row, col);
            board.tiles[idx] = Tile::from_str(&hex_char.to_string()).map_err(|_| ())?;
        }
        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board_from_str(s: &str) -> Board {
        s.parse().expect("Failed to parse board")
    }

    #[test]
    fn test_left_index_mapping() {
        let board = Board::new(4);
        let mapping = board.index_map.get(&Direction::Left).unwrap();

        assert_eq!(mapping[0], vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
        assert_eq!(mapping[1], vec![(1, 0), (1, 1), (1, 2), (1, 3)]);
        assert_eq!(mapping[2], vec![(2, 0), (2, 1), (2, 2), (2, 3)]);
        assert_eq!(mapping[3], vec![(3, 0), (3, 1), (3, 2), (3, 3)]);
    }

    #[test]
    fn test_right_index_mapping() {
        let board = Board::new(4);
        let mapping = board.index_map.get(&Direction::Right).unwrap();

        assert_eq!(mapping[0], vec![(0, 3), (0, 2), (0, 1), (0, 0)]);
        assert_eq!(mapping[1], vec![(1, 3), (1, 2), (1, 1), (1, 0)]);
        assert_eq!(mapping[2], vec![(2, 3), (2, 2), (2, 1), (2, 0)]);
        assert_eq!(mapping[3], vec![(3, 3), (3, 2), (3, 1), (3, 0)]);
    }

    #[test]
    fn test_up_index_mapping() {
        let board = Board::new(4);
        let mapping = board.index_map.get(&Direction::Up).unwrap();

        assert_eq!(mapping[0], vec![(0, 0), (1, 0), (2, 0), (3, 0)]);
        assert_eq!(mapping[1], vec![(0, 1), (1, 1), (2, 1), (3, 1)]);
        assert_eq!(mapping[2], vec![(0, 2), (1, 2), (2, 2), (3, 2)]);
        assert_eq!(mapping[3], vec![(0, 3), (1, 3), (2, 3), (3, 3)]);
    }

    #[test]
    fn test_down_index_mapping() {
        let board = Board::new(4);
        let mapping = board.index_map.get(&Direction::Down).unwrap();

        assert_eq!(mapping[0], vec![(3, 0), (2, 0), (1, 0), (0, 0)]);
        assert_eq!(mapping[1], vec![(3, 1), (2, 1), (1, 1), (0, 1)]);
        assert_eq!(mapping[2], vec![(3, 2), (2, 2), (1, 2), (0, 2)]);
        assert_eq!(mapping[3], vec![(3, 3), (2, 3), (1, 3), (0, 3)]);
    }

    #[test]
    fn test_slide_and_merge() {
        struct TestCase {
            input: &'static str,
            direction: Direction,
            expected: &'static str,
            moved: bool,
            name: &'static str,
        }

        let cases = [
            TestCase {input: "1000100010001000", expected: "1000100010001000", direction: Direction::Left, moved: false, name: "no slide no merge"},
            TestCase {input: "0001000100010001", expected: "0001000100010001", direction: Direction::Right, moved: false, name: "no slide no merge"},
            TestCase {input: "1111000000000000", expected: "1111000000000000", direction: Direction::Up, moved: false, name: "no slide no merge"},
            TestCase {input: "0000000000001111", expected: "0000000000001111", direction: Direction::Down, moved: false, name: "no slide no merge"},
            
            TestCase {input: "0001000100010001", expected: "1000100010001000", direction: Direction::Left, moved: true, name: "just slide no merge"},
            TestCase {input: "1000100010001000", expected: "0001000100010001", direction: Direction::Right, moved: true, name: "just slide no merge"},
            TestCase {input: "0000000000001111", expected: "1111000000000000", direction: Direction::Up, moved: true, name: "just slide no merge"},
            TestCase {input: "1111000000000000", expected: "0000000000001111", direction: Direction::Down, moved: true, name: "just slide no merge"},

            TestCase {input: "1000010000100001", expected: "1000100010001000", direction: Direction::Left, moved: true, name: "just slide diagonal"},
            TestCase {input: "1000010000100001", expected: "0001000100010001", direction: Direction::Right, moved: true, name: "just slide diagonal"},
            TestCase {input: "1000010000100001", expected: "1111000000000000", direction: Direction::Up, moved: true, name: "just slide diagonal"},
            TestCase {input: "1000010000100001", expected: "0000000000001111", direction: Direction::Down, moved: true, name: "just slide diagonal"},
            
            TestCase {input: "1100220033004400", expected: "2000300040005000", direction: Direction::Left, moved: true, name: "slide and merge"},
            TestCase {input: "0011002200330044", expected: "0002000300040005", direction: Direction::Right, moved: true, name: "slide and merge"},
            TestCase {input: "1234123400000000", expected: "2345000000000000", direction: Direction::Up, moved: true, name: "slide and merge"},
            TestCase {input: "0000000012341234", expected: "0000000000002345", direction: Direction::Down, moved: true, name: "slide and merge"},

            TestCase {input: "1111111111111111", expected: "2200220022002200", direction: Direction::Left, moved: true, name: "merge twice"},
            TestCase {input: "1111111111111111", expected: "0022002200220022", direction: Direction::Right, moved: true, name: "merge twice"},
            TestCase {input: "1111111111111111", expected: "2222222200000000", direction: Direction::Up, moved: true, name: "merge twice"},
            TestCase {input: "1111111111111111", expected: "0000000022222222", direction: Direction::Down, moved: true, name: "merge twice"},

            TestCase {input: "1110101111010111", expected: "2100210021002100", direction: Direction::Left, moved: true, name: "gap invaraible"},
            TestCase {input: "1110101111010111", expected: "0012001200120012", direction: Direction::Right, moved: true, name: "gap invaraible"},
            TestCase {input: "1110101111010111", expected: "2222111100000000", direction: Direction::Up, moved: true, name: "gap invaraible"},
            TestCase {input: "1110101111010111", expected: "0000000011112222", direction: Direction::Down, moved: true, name: "gap invaraible"},

            TestCase {input: "0000000000000000", expected: "0000000000000000", direction: Direction::Left, moved: false, name: "no move empty"},
            TestCase {input: "0000000000000000", expected: "0000000000000000", direction: Direction::Right, moved: false, name: "no move empty"},
            TestCase {input: "0000000000000000", expected: "0000000000000000", direction: Direction::Up, moved: false, name: "no move empty"},
            TestCase {input: "0000000000000000", expected: "0000000000000000", direction: Direction::Down, moved: false, name: "no move empty"},

            TestCase {input: "1234234134124123", expected: "1234234134124123", direction: Direction::Left, moved: false, name: "no move full"},
            TestCase {input: "1234234134124123", expected: "1234234134124123", direction: Direction::Right, moved: false, name: "no move full"},
            TestCase {input: "1234234134124123", expected: "1234234134124123", direction: Direction::Up, moved: false, name: "no move full"},
            TestCase {input: "1234234134124123", expected: "1234234134124123", direction: Direction::Down, moved: false, name: "no move full"},
        ];

        for case in &cases {
            let mut board = board_from_str(case.input);
            let moved = board.slide_and_merge(case.direction);
            assert_eq!(board.to_string(), case.expected, "{} --{}--> {} ({})", case.input, case.direction, case.expected, case.name);
            assert_eq!(moved, case.moved, "{} --{}--> {} ({})", case.input, case.direction, case.moved, case.name);
        }
    }

    #[test]
    fn test_board_serialization() {
        let board = board_from_str("123456789A000000");

        let board_str = board.to_string();
        let restored_board: Board = board_str.parse().expect("Failed to parse board");

        assert_eq!(board.tiles, restored_board.tiles);
    }
}
