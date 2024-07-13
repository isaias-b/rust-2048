use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::action::Action;
use crate::direction::Direction;
use crate::tile::position::{LineTraversals, Position};
use crate::tile::value::{Value, MAX_TILE_VALUE};
use crate::tile::Tile;

#[derive(Debug, Clone)]
pub struct Board {
    pub size: usize,
    pub tiles: HashMap<Position, Value>,
    pub traversal_map: HashMap<Direction, LineTraversals>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let mut tiles = HashMap::new();
        for row in 0..size {
            for col in 0..size {
                let pos = Position { row, col };
                tiles.insert(pos, Value::Empty);
            }
        }
        Self {
            size,
            tiles,
            traversal_map: Position::generate_traversal_map(size),
        }
    }

    pub fn set_value(&mut self, p: &Position, value: Value) {
        self.tiles.insert(p.clone(), value);
    }

    pub fn get_value(&self, p: &Position) -> Value {
        self.tiles.get(p).unwrap().clone()
    }

    pub fn get_tile(&self, p: &Position) -> Tile {
        Tile {
            value: self.get_value(p),
            position: p.clone(),
        }
    }

    pub fn plan_spawn_random_tile(&self, rng: &mut ChaCha8Rng) -> Option<Action> {
        let mut empty_positions = vec![];
        self.traversal_map
            .get(&Direction::Left)
            .unwrap()
            .iter()
            .for_each(|line| {
                line.iter().for_each(|pos| {
                    if self.get_value(pos) == Value::Empty {
                        empty_positions.push(pos.clone());
                    }
                });
            });
        let has_empty_positions = !empty_positions.is_empty();
        if has_empty_positions {
            let rand_position = empty_positions[rng.gen_range(0..empty_positions.len())];
            let rand_value = if rng.gen_bool(0.9) { 2 } else { 4 };
            let rand_tile = Value::Number(rand_value);

            Some(Action::SpawnRandomTile(Tile {
                value: rand_tile,
                position: rand_position,
            }))
        } else {
            None
        }
    }

    pub fn apply(&mut self, event: Action) {
        match event {
            Action::SpawnRandomTile(tile) => {
                self.set_value(&tile.position, tile.value);
            }
            Action::SlideTile(tile, to) => {
                self.set_value(&tile.position, Value::Empty);
                self.set_value(&to, tile.value);
            }
            Action::MergeTiles(tile1, tile2, to, value) => {
                self.set_value(&tile1.position, Value::Empty);
                self.set_value(&tile2.position, Value::Empty);
                self.set_value(&to, value);
            }
        }
    }

    pub fn plan_slide_and_merge(&self, direction: &Direction) -> Vec<Action> {
        let line_traversals = self.traversal_map.get(direction).unwrap();
        let mut events = vec![];

        for line_traversal in line_traversals {
            let es = self.slide_and_merge_line(line_traversal);
            events.extend(es);
        }

        events
    }

    pub fn slide_and_merge_line(&self, line_traversal: &Vec<Position>) -> Vec<Action> {
        let mut events = vec![];
        let mut board_clone = self.clone();

        let mut focus_idx = 0;
        let mut prev: Option<(usize, u32)> = None;
        let mut deferred_slide: Option<Action> = None;
        let mut deferred: Option<(Tile, Position)> = None;

        for current_cell in line_traversal {
            let focus_cell = &line_traversal[focus_idx];
            let can_slide = current_cell != focus_cell; // means there were empty tiles up to this point

            let current_value =
                if let Value::Number(value) = *board_clone.tiles.get(&current_cell).unwrap() {
                    value
                } else {
                    continue; // Skip empty cells
                };

            if let Some((prev_idx, prev_value)) = prev {
                let can_merge = prev_value == current_value && prev_value < MAX_TILE_VALUE;
                let prev_cell = line_traversal[prev_idx];

                if can_merge {
                    let (tile1, pos) = if let Some((tile, pos)) = deferred.take() {
                        deferred_slide = None; // ignore deferred slide because we're turning it into a merge
                        (tile, pos)
                    } else {
                        (board_clone.get_tile(&prev_cell), prev_cell.clone())
                    };
                    let tile2 = board_clone.get_tile(&current_cell);
                    let value = tile1.value.merge(tile2.value);
                    let event = Action::MergeTiles(tile1, tile2, pos, value);
                    board_clone.apply(event.clone());
                    events.push(event);
                    prev = None;
                } else {
                    if can_slide {
                        if let Some((tile, pos)) = deferred.take() {
                            let event = Action::SlideTile(tile, pos);
                            board_clone.apply(event.clone()); // complete deferred slide before sliding the current tile
                            events.push(event);
                        }
                        let tile = board_clone.get_tile(&current_cell);
                        let event = Action::SlideTile(tile, focus_cell.clone());
                        deferred = Some((tile, focus_cell.clone()));
                        deferred_slide = Some(event.clone());
                        // board_clone.apply(event.clone());
                        // events.push(event);
                    }
                    prev = Some((focus_idx, current_value));
                    focus_idx += 1;
                }
            } else {
                if can_slide {
                    let tile = board_clone.get_tile(&current_cell);
                    let event = Action::SlideTile(tile, focus_cell.clone());
                    deferred = Some((tile, focus_cell.clone()));
                    deferred_slide = Some(event.clone());
                }
                prev = Some((focus_idx, current_value)); // focus_idx seems off here
                focus_idx += 1;
            }
        }
        if let Some((tile, pos)) = deferred.take() {
            let event = Action::SlideTile(tile, pos);
            board_clone.apply(event.clone());
            events.push(event);
        }
        return events;
    }

    pub fn slide_and_merge(&mut self, direction: Direction) -> bool {
        let events = self.plan_slide_and_merge(&direction);
        let moved = !events.is_empty();

        for event in events {
            self.apply(event);
        }

        moved
    }

    // pub fn spawn_random_tile(&mut self) {
    //     if let Some(event) = self.plan_spawn_random_tile() {
    //         self.apply(event);
    //     }
    // }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.traversal_map
            .get(&Direction::Left)
            .unwrap()
            .iter()
            .for_each(|line| {
                line.iter().for_each(|pos| {
                    write!(f, "{}", self.get_value(pos)).unwrap();
                });
            });
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
            let pos = &Position { row, col };
            board.set_value(pos, Value::from_str(&hex_char.to_string()).map_err(|_| ())?);
        }
        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use bevy::reflect::Reflect;

    use super::*;

    fn board_from_str(s: &str) -> Board {
        s.parse().expect("Failed to parse board")
    }
    struct TestCase {
        in_board: &'static str,
        in_direction: Direction,
        board: &'static str,
        n_events: usize,
        moved: bool,
        name: &'static str,
    }

    #[test]
    fn test_plan_slide_and_merge() {
        for case in &CASES {
            let board = board_from_str(case.in_board);
            let events = board.plan_slide_and_merge(&case.in_direction);
            assert_eq!(
                events.len(),
                case.n_events,
                "{} --{}--> {} events ({})",
                case.in_board,
                case.in_direction,
                case.n_events,
                case.name
            );
        }
    }

    #[test]
    fn test_slide_and_merge_line() {
        let mut in_board = board_from_str("1110000000000000");
        let in_direction = Direction::Right;
        let line_traversal = in_board.traversal_map.get(&in_direction).unwrap()[0].clone();
        let events = in_board.slide_and_merge_line(&line_traversal);
        let mut board = in_board.clone();
        board.apply(events.get(0).unwrap().clone());
        board.apply(events.get(1).unwrap().clone());
        if let Action::MergeTiles(tile1, tile2, to, value) = (*events.get(0).unwrap()) {
            assert_eq!(tile1.value, Value::Number(2));
            assert_eq!(tile2.value, Value::Number(2));
            assert_eq!(to, Position { row: 0, col: 3 });
            assert_eq!(value, Value::Number(4));
        }
        if let Action::SlideTile(tile, to) = (*events.get(1).unwrap()) {
            assert_eq!(tile.value, Value::Number(2));
            assert_eq!(to, Position { row: 0, col: 2 });
        }
        assert_eq!(events.len(), 2);
        assert_eq!(board.to_string(), "0012000000000000");
    }

    #[test]
    fn test_slide_and_merge_line_wild_case() {
        let mut in_board = board_from_str("2000200010000000");
        let in_direction = Direction::Down;

        let mut board = in_board.clone();
        board.slide_and_merge(in_direction);
        assert_eq!(board.to_string(), "0000000030001000");

        let line_traversal = in_board.traversal_map.get(&in_direction).unwrap()[0].clone();
        let events = in_board.slide_and_merge_line(&line_traversal);
        assert_eq!(events.len(), 2);

        println!("{:?}", events);
        if let Action::SlideTile(tile, to) = (*events.get(0).unwrap()) {
            assert_eq!(tile.value, Value::Number(2));
            assert_eq!(to, Position { row: 3, col: 0 });
        }
        if let Action::MergeTiles(tile1, tile2, to, value) = (*events.get(1).unwrap()) {
            assert_eq!(tile1.value, Value::Number(4));
            assert_eq!(tile2.value, Value::Number(4));
            assert_eq!(tile1.position, Position { row: 1, col: 0 });
            assert_eq!(tile2.position, Position { row: 0, col: 0 });
            assert_eq!(to, Position { row: 2, col: 0 });
            assert_eq!(value, Value::Number(8));
        }
    }

    #[test]
    fn test_slide_and_merge_line_slide_and_merge_left() {
        let mut in_board = board_from_str("0122000000000000");
        let in_direction = Direction::Left;
        let line_traversal = in_board.traversal_map.get(&in_direction).unwrap()[0].clone();
        let events = in_board.slide_and_merge_line(&line_traversal);

        let mut board = in_board.clone();
        for event in events.iter() {
            board.apply(event.clone());
        }
        assert_eq!(board.to_string(), "1300000000000000");

        if let Action::SlideTile(tile, to) = (*events.get(0).unwrap()) {
            assert_eq!(tile.value, Value::Number(2));
            assert_eq!(tile.position, Position { row: 0, col: 1 });
            assert_eq!(to, Position { row: 0, col: 0 });
        }
        if let Action::MergeTiles(tile1, tile2, to, value) = (*events.get(1).unwrap()) {
            println!("{:?}", *events.get(1).unwrap());
            assert_eq!(tile1.value, Value::Number(4));
            assert_eq!(tile2.value, Value::Number(4));
            assert_eq!(value, Value::Number(8));
            assert_eq!(tile1.position, Position { row: 0, col: 2 });
            assert_eq!(tile2.position, Position { row: 0, col: 3 });
            assert_eq!(to, Position { row: 0, col: 1 });
        }
        assert_eq!(events.len(), 2);
    }

    #[rustfmt::skip]
        const CASES: [TestCase; 40] = [
            // TestCase { in_board: "3301100000000010", board: "0041000100000001", n_events: 4, in_direction: Direction::Right,  moved: true, name: "wild case" },
            // TestCase { in_board: "2300200010010000", board: "0000000030001301", n_events: 4, in_direction: Direction::Down,  moved: true, name: "wild case" },
            TestCase { in_board: "0000011001100000", board: "0000200020000000", n_events: 2, in_direction: Direction::Left,  moved: true, name: "gap corner" },
            TestCase { in_board: "0000011001100000", board: "0000000200020000", n_events: 2, in_direction: Direction::Right, moved: true, name: "gap corner" },
            TestCase { in_board: "0000011001100000", board: "0220000000000000", n_events: 2, in_direction: Direction::Up,    moved: true, name: "gap corner" },
            TestCase { in_board: "0000011001100000", board: "0000000000000220", n_events: 2, in_direction: Direction::Down,  moved: true, name: "gap corner" },
            TestCase { in_board: "0122000000000000", board: "1300000000000000", n_events: 2, in_direction: Direction::Left,  moved: true, name: "slide and merge" },
            TestCase { in_board: "2210000000000000", board: "0031000000000000", n_events: 2, in_direction: Direction::Right, moved: true, name: "slide and merge" },
            TestCase { in_board: "0000100020002000", board: "1000300000000000", n_events: 2, in_direction: Direction::Up,    moved: true, name: "slide and merge" },
            TestCase { in_board: "2000200010000000", board: "0000000030001000", n_events: 2, in_direction: Direction::Down,  moved: true, name: "slide and merge" },
            TestCase { in_board: "0000000000000000", board: "0000000000000000", n_events: 0, in_direction: Direction::Left,  moved: false, name: "no move empty" },
            TestCase { in_board: "0000000000000000", board: "0000000000000000", n_events: 0, in_direction: Direction::Right, moved: false, name: "no move empty" },
            TestCase { in_board: "0000000000000000", board: "0000000000000000", n_events: 0, in_direction: Direction::Up,    moved: false, name: "no move empty" },
            TestCase { in_board: "0000000000000000", board: "0000000000000000", n_events: 0, in_direction: Direction::Down,  moved: false, name: "no move empty" },
            TestCase { in_board: "1234234134124123", board: "1234234134124123", n_events: 0, in_direction: Direction::Left,  moved: false, name: "no move full" },
            TestCase { in_board: "1234234134124123", board: "1234234134124123", n_events: 0, in_direction: Direction::Right, moved: false, name: "no move full" },
            TestCase { in_board: "1234234134124123", board: "1234234134124123", n_events: 0, in_direction: Direction::Up,    moved: false, name: "no move full" },
            TestCase { in_board: "1234234134124123", board: "1234234134124123", n_events: 0, in_direction: Direction::Down,  moved: false, name: "no move full" },
            TestCase { in_board: "1000100010001000", board: "1000100010001000", n_events: 0, in_direction: Direction::Left,  moved: false, name: "no slide no merge" },
            TestCase { in_board: "0001000100010001", board: "0001000100010001", n_events: 0, in_direction: Direction::Right, moved: false, name: "no slide no merge" },
            TestCase { in_board: "1111000000000000", board: "1111000000000000", n_events: 0, in_direction: Direction::Up,    moved: false, name: "no slide no merge" },
            TestCase { in_board: "0000000000001111", board: "0000000000001111", n_events: 0, in_direction: Direction::Down,  moved: false, name: "no slide no merge" },
            TestCase { in_board: "0001000100010001", board: "1000100010001000", n_events: 4, in_direction: Direction::Left,  moved: true, name: "just slide no merge" },
            TestCase { in_board: "1000100010001000", board: "0001000100010001", n_events: 4, in_direction: Direction::Right, moved: true, name: "just slide no merge" },
            TestCase { in_board: "0000000000001111", board: "1111000000000000", n_events: 4, in_direction: Direction::Up,    moved: true, name: "just slide no merge" },
            TestCase { in_board: "1111000000000000", board: "0000000000001111", n_events: 4, in_direction: Direction::Down,  moved: true, name: "just slide no merge" },
            TestCase { in_board: "1000010000100001", board: "1000100010001000", n_events: 3, in_direction: Direction::Left,  moved: true, name: "just slide diagonal" },
            TestCase { in_board: "1000010000100001", board: "0001000100010001", n_events: 3, in_direction: Direction::Right, moved: true, name: "just slide diagonal" },
            TestCase { in_board: "1000010000100001", board: "1111000000000000", n_events: 3, in_direction: Direction::Up,    moved: true, name: "just slide diagonal" },
            TestCase { in_board: "1000010000100001", board: "0000000000001111", n_events: 3, in_direction: Direction::Down,  moved: true, name: "just slide diagonal" },
            TestCase { in_board: "1100220033004400", board: "2000300040005000", n_events: 4, in_direction: Direction::Left,  moved: true, name: "just merge" },
            TestCase { in_board: "0011002200330044", board: "0002000300040005", n_events: 4, in_direction: Direction::Right, moved: true, name: "just merge" },
            TestCase { in_board: "1234123400000000", board: "2345000000000000", n_events: 4, in_direction: Direction::Up,    moved: true, name: "just merge" },
            TestCase { in_board: "0000000012341234", board: "0000000000002345", n_events: 4, in_direction: Direction::Down,  moved: true, name: "just merge" },
            TestCase { in_board: "1110101111010111", board: "2100210021002100", n_events: 8, in_direction: Direction::Left,  moved: true, name: "gap invaraible" },
            TestCase { in_board: "1110101111010111", board: "0012001200120012", n_events: 8, in_direction: Direction::Right, moved: true, name: "gap invaraible" },
            TestCase { in_board: "1110101111010111", board: "2222111100000000", n_events: 8, in_direction: Direction::Up,    moved: true, name: "gap invaraible" },
            TestCase { in_board: "1110101111010111", board: "0000000011112222", n_events: 8, in_direction: Direction::Down,  moved: true, name: "gap invaraible" },
            TestCase { in_board: "1111111111111111", board: "2200220022002200", n_events: 8, in_direction: Direction::Left,  moved: true, name: "merge twice" },
            TestCase { in_board: "1111111111111111", board: "0022002200220022", n_events: 8, in_direction: Direction::Right, moved: true, name: "merge twice" },
            TestCase { in_board: "1111111111111111", board: "2222222200000000", n_events: 8, in_direction: Direction::Up,    moved: true, name: "merge twice" },
            TestCase { in_board: "1111111111111111", board: "0000000022222222", n_events: 8, in_direction: Direction::Down,  moved: true, name: "merge twice" },
        ];

    #[test]
    fn test_slide_and_merge() {
        for case in &CASES {
            let mut board = board_from_str(case.in_board);
            let moved = board.slide_and_merge(case.in_direction);
            assert_eq!(
                board.to_string(),
                case.board,
                "{} --{}--> {} ({})",
                case.in_board,
                case.in_direction,
                case.board,
                case.name
            );
            assert_eq!(
                moved, case.moved,
                "{} --{}--> {} ({})",
                case.in_board, case.in_direction, case.moved, case.name
            );
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
