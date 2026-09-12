#[allow(unused)]
use std::error::Error;
use std::option::Option;
use std::vec::Vec;
use std::string::String;
use std::fmt;
use std::collections::{
    HashMap,
    hash_map::Entry,
};

use array2d::Array2D;
use log::{
    info,
    warn,
};

type Location = (usize, usize);

#[derive(Copy,Clone,Debug,PartialEq)]
enum BoardError {
    CellOccupied(Location, Player),
    LocationInvalid(Location),
    NoSuchPlayer(Player),
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardError::CellOccupied(loc, player) => write!(f, "Cell at ({}, {}) is occupied by player {}.", loc.0, loc.1, player),
            BoardError::LocationInvalid(loc) => write!(f, "Cell at ({}, {}) doesn’t exist.", loc.0, loc.1),
            BoardError::NoSuchPlayer(player) => write!(f, "Player {} isn’t playing.", player),
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
enum CellPos {
    Corner,
    Edge,
    Bulk,
}

impl CellPos {
    fn limit(&self) -> u8 {
        match self {
            &CellPos::Corner 	=> 1,
            &CellPos::Edge 		=> 2,
            &CellPos::Bulk 		=> 3,
        }
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
enum Player {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::One => write!(f, "One"),
            Player::Two => write!(f, "Two"),
            Player::Three => write!(f, "Three"),
            Player::Four => write!(f, "Four"),
            Player::Five => write!(f, "Five"),
            Player::Six => write!(f, "Six"),
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
struct Cell {
    position: CellPos,
    contents: u8,
    player: Player,
}

impl Cell {
    fn new(position: CellPos) -> Self {
        Cell {
            position,
            contents: 0,
            player: Player::One,
        }
    }

    fn is_bursting(&self) -> bool {
        self.contents > self.position.limit()
    }

    fn increment(&mut self) {
        self.contents += 1;
    }

    fn clear(&mut self) {
        self.contents -= self.position.limit() + 1 ;
    }

    fn set_player(&mut self, player: &Player) {
        self.player = *player;
    }

    fn can_move_player(&self, player: &Player) -> bool {
        (self.contents == 0) || (&self.player == player)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.contents {
            1 => write!(f, "1"),
            2 => write!(f, "2"),
            3 => write!(f, "3"),
            _ => write!(f, " "),
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
struct Queue {
    map: HashMap<Location, u8>,
}

impl Queue {
    fn new() -> Self {
        Queue { map: HashMap::new(), }
    }

    fn append(&mut self, board: &Board, loc: Location) {
        let limit = board.get_type(&loc).limit() + 1;

        match self.map.entry(loc) {
            Entry::Vacant(_) => {self.map.insert(loc, 1);},
            Entry::Occupied(mut entry) => {
                let val = entry.get_mut();
                if *val < limit { *val += 1; }
            }
        }
    }

    fn from_vec(vec: Vec<Location>, board: &Board) -> Self {
        let mut queue = Self::new();
        let _ = vec.into_iter()
            .map(|loc| queue.append(board, loc));
        queue
    }

    fn to_vec(&self) -> Vec<Location> {
        let mut vec = Vec::new();
        for key in self.map.keys() {
            for i in 0u8..*self.map.get(key).unwrap() {
                vec.push(*key);
            }
        }
        vec
    }
}

#[derive(Clone,Debug,PartialEq)]
struct Board {
    grid: Array2D<Cell>,
    rows: usize,
    cols: usize,
    players: Vec<Player>,
}

impl Board {
    fn new(rows: usize, cols: usize, players: Vec<Player>) -> Self {
        let mut grid = Array2D::filled_with(
            Cell::new(CellPos::Bulk),
            rows-2,
            cols-2,
        ).as_rows();

        for row in &mut grid {
            row.insert(0, Cell::new(CellPos::Edge));
            row.push(Cell::new(CellPos::Edge));
        }

        let mut edge_row = Vec::new();
        for i in 0..cols { edge_row.push(Cell::new(CellPos::Edge)); }

        grid.insert(
            0,
            edge_row.clone()
        );
        grid.push(edge_row.clone());

        grid[0][0] = Cell::new(CellPos::Corner);
        grid[0][cols-1] = Cell::new(CellPos::Corner);
        grid[rows-1][0] = Cell::new(CellPos::Corner);
        grid[rows-1][cols-1] = Cell::new(CellPos::Corner);

        Board {
            grid: Array2D::from_rows(&grid).unwrap(),
            rows: rows-1,
            cols: cols-1,
            players,
        }
    }

    fn get_neighbours(&self, location: &Location) -> Option<Vec<Location>> {
        let row_lim = self.rows;
        let col_lim = self.cols;

        let (row, col) = *location;

        if (row > row_lim) || (col > col_lim) {
            return None;
        }

        if row == 0 {
            if col == 0 {
                Some(vec![
                    (0, 1),
                    (1, 0),
                ])
            } else if col == col_lim {
                Some(vec![
                    (0, col_lim-1),
                    (1, col_lim),
                ])
            } else {
                Some(vec![
                    (0, col-1),
                    (0, col+1),
                    (1, col),
                ])
            }
        } else if row == row_lim {
            if col == 0 {
                Some(vec![
                    (row_lim-1, 0),
                    (row_lim, 1),
                ])
            } else if col == col_lim {
                Some(vec![
                    (row_lim-1, col_lim),
                    (row_lim, col_lim-1),
                ])
            } else {
                Some(vec![
                    (row_lim-1, col),
                    (row_lim, col-1),
                    (row_lim, col+1),
                ])
            }
        } else {
            if col == 0 {
                Some(vec![
                    (row-1, 0),
                    (row, 1),
                    (row+1, 0),
                ])
            } else if col == col_lim {
                Some(vec![
                    (row-1, col_lim),
                    (row, col_lim-1),
                    (row+1, col_lim),
                ])
            } else {
                Some(vec![
                    (row-1, col),
                    (row, col-1),
                    (row, col+1),
                    (row+1, col),
                ])
            }
        }
    }

    fn get_type(&self, loc: &Location) -> CellPos {
        let (row, col) = *loc;
        self.grid.get(row, col).expect("Invalid location.").position
    }

    fn capture_cell(&mut self, player: &Player, location: &Location) -> bool {
        let (row, col) = *location;

        let mut new_cell = *self.grid.get(row, col)
            .expect("Invalid location.");
        new_cell.increment();
        new_cell.set_player(player);
        let burst = new_cell.is_bursting();
        if burst { new_cell.clear(); }
        let _ = self.grid.set(row, col, new_cell);
        burst
    }

    fn has_lost(&self) -> Vec<Player> {
        let mut cell_counts = HashMap::new();
        for player in self.players.iter() {
            cell_counts.insert(*player, 0u16);
        }

        self.grid.clone()
            .elements_row_major_iter()
            .for_each(|cell| { cell_counts.entry(cell.player).and_modify(|count| *count += 1); });

        let mut losers = Vec::new();

        cell_counts.iter().for_each(|(player, count)| {
                if *count == 0 {
                    losers.push(*player);
                }
            });

        losers
    }

    fn burst(&mut self, player: &Player, location: Location) {
        let mut queue = self.get_neighbours(&location).unwrap();
        while !queue.is_empty() {
            let mut extra_q = Queue::new();
            for loc in &queue {
                if self.capture_cell(player, loc) {
                    for i in self.get_neighbours(loc).unwrap() {
                        extra_q.append(self, i);
                    }
                }
            }
            warn!("Adding {:?}", extra_q);
            queue = extra_q.to_vec();
        }
    }

    fn try_move(&mut self, player: &Player, location: Location) -> Result<(), BoardError> {
        let (row, col) = location;
        match self.grid.get(row, col) {
            Some(cell) => if self.players.contains(player) {
                if cell.can_move_player(player) {
                    info!("Player {:?} moving to {:?}", player, location);
                    let mut new_cell = *cell;
                    new_cell.set_player(player);
                    new_cell.increment();
                    let burst = new_cell.is_bursting();
                    if burst {
                        new_cell.clear();
                    }

                    let _ = self.grid.set(row, col, new_cell);

                    if burst {
                        self.burst(player, location);
                        Ok(())
                    } else {
                        Ok(())
                    }
                } else {
                    Err(BoardError::CellOccupied(location, cell.player))
                }
            } else {
                Err(BoardError::NoSuchPlayer(*player))
            }
            None => Err(BoardError::LocationInvalid(location))
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = self.grid.as_rows();
        let mut inter_str = String::from("├─");
        let mut top_str = String::from("╭─");
        let mut bottom_str = String::from("╰─");

        for _i in 0..board[0].len()-1 {
            inter_str.push_str("──┼─");
            top_str.push_str("──┬─");
            bottom_str.push_str("──┴─");
        }

        inter_str.push_str("──┤");
        top_str.push_str("──╮");
        bottom_str.push_str("──╯");

        let mut board_str = top_str;

        let mut not_first = false;

        board.into_iter()
            .map(|row| {
                let mut row_str = String::from("│");
                row.iter()
                    .for_each(|cell| row_str.push_str(format!(" {} │", cell).as_str()));
                row_str
            })
            .for_each(|row_str| {
                if not_first {
                    board_str = format!("{}\n{}\n{}", board_str, inter_str, row_str)
                } else {
                    board_str = format!("{}\n{}", board_str, row_str);
                    not_first = true
                }
            });

        write!(f, "{}\n{}", board_str, bottom_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_logger::SimpleLogger;
    use std::{println as info, println as warn};

    #[test]
    fn will_burst() {
        let corner_a = Cell {
            position: CellPos::Corner,
            contents: 1,
            player: Player::One,
        };
        let corner_b = Cell {
            position: CellPos::Corner,
            contents: 2,
            player: Player::One,
        };
        let edge_a = Cell {
            position: CellPos::Edge,
            contents: 2,
            player: Player::One,
        };
        let edge_b = Cell {
            position: CellPos::Edge,
            contents: 3,
            player: Player::One,
        };
        let bulk_a = Cell {
            position: CellPos::Bulk,
            contents: 2,
            player: Player::One,
        };
        let bulk_b = Cell {
            position: CellPos::Bulk,
            contents: 4,
            player: Player::One,
        };

        assert_eq!(corner_a.is_bursting(), false);
        assert_eq!(corner_b.is_bursting(), true);
        assert_eq!(edge_a.is_bursting(), false);
        assert_eq!(edge_b.is_bursting(), true);
        assert_eq!(bulk_a.is_bursting(), false);
        assert_eq!(bulk_b.is_bursting(), true);
    }

    #[test]
    fn board_edges() {
        SimpleLogger::new()
            .init()
            .unwrap();

        let board = Board::new(4, 5, vec![Player::One,Player::Two]);

        let mut row_vec = board.grid.as_rows();
        let mut col_vec = board.grid.as_columns();

        let rows = row_vec.len();
        let cols = col_vec.len();

        let corner_00 = row_vec[0].remove(0);
        let corner_01 = row_vec[0].remove(cols-2);

        let corner_10 = row_vec[rows-1].remove(0);
        let corner_11 = row_vec[rows-1].remove(cols-2);

        col_vec[0].remove(0);
        col_vec[0].remove(rows-2);

        col_vec[cols-1].remove(0);
        col_vec[cols-1].remove(rows-2);

        assert_eq!(corner_00.position, CellPos::Corner);
        assert_eq!(corner_01.position, CellPos::Corner);
        assert_eq!(corner_10.position, CellPos::Corner);
        assert_eq!(corner_11.position, CellPos::Corner);

        row_vec[0].clone()
            .iter()
            .zip(1..rows-1)
            .for_each(|(cell, i)| assert_eq!(cell.position, CellPos::Edge, "0, {}", i));

        row_vec[rows-1].clone()
            .iter()
            .zip(1..rows-1)
            .for_each(|(cell, i)| assert_eq!(cell.position, CellPos::Edge, "{}, {}", rows-1, i));

        col_vec[0].clone()
            .iter()
            .zip(1..cols-1)
            .for_each(|(cell, i)| assert_eq!(cell.position, CellPos::Edge, "{}, 0", i));

        col_vec[cols-1].clone()
            .iter()
            .zip(1..cols-1)
            .for_each(|(cell, i)| assert_eq!(cell.position, CellPos::Edge, "{}, {}", i, cols-1));
    }

    #[test]
    fn check_neighbours() {
        let board = Board::new(6, 8, vec![Player::One,Player::Two]);

        assert_eq!(board.get_neighbours(&(0, 0)), Some(vec![
            (0, 1),
            (1, 0),
        ]));

        assert_eq!(board.get_neighbours(&(5, 0)), Some(vec![
            (4, 0),
            (5, 1),
        ]));

        assert_eq!(board.get_neighbours(&(5, 7)), Some(vec![
            (4, 7),
            (5, 6),
        ]));

        assert_eq!(board.get_neighbours(&(0, 7)), Some(vec![
            (0, 6),
            (1, 7),
        ]));

        assert_eq!(board.get_neighbours(&(0, 1)), Some(vec![
            (0, 0),
            (0, 2),
            (1, 1),
        ]));

        assert_eq!(board.get_neighbours(&(1, 0)), Some(vec![
            (0, 0),
            (1, 1),
            (2, 0),
        ]));

        assert_eq!(board.get_neighbours(&(5, 1)), Some(vec![
            (4, 1),
            (5, 0),
            (5, 2),
        ]));

        assert_eq!(board.get_neighbours(&(1, 7)), Some(vec![
            (0, 7),
            (1, 6),
            (2, 7),
        ]));

        assert_eq!(board.get_neighbours(&(2, 3)), Some(vec![
            (1, 3),
            (2, 2),
            (2, 4),
            (3, 3),
        ]));

        assert_eq!(board.get_neighbours(&(6, 8)), None);
        assert_eq!(board.get_neighbours(&(6, 7)), None);
        assert_eq!(board.get_neighbours(&(5, 8)), None);
    }

    #[test]
    fn player_equality() {
        assert_eq!((Player::One == Player::Two), false);
    }

    #[test]
    fn can_move() {
        let mut cell = Cell::new(CellPos::Bulk);

        assert_eq!(cell.can_move_player(&Player::One), true);
        assert_eq!(cell.can_move_player(&Player::Two), true);

        cell.set_player(&Player::Two);
        cell.increment();

        assert_eq!(cell.can_move_player(&Player::One), false);
        assert_eq!(cell.can_move_player(&Player::Two), true);
    }

    #[test]
    fn board_move() {
        let mut board = Board::new(4, 5, vec![Player::One,Player::Two]);

        assert_eq!(board.try_move(&Player::One, (0, 0)), Ok(()));
        assert_eq!(board.try_move(&Player::Two, (0, 0)), Err(BoardError::CellOccupied((0, 0), Player::One)));
        assert_eq!(board.try_move(&Player::One, (0, 0)), Ok(()));
        assert_eq!(board.try_move(&Player::One, (4, 3)), Err(BoardError::LocationInvalid((4, 3))));

        let mut check_board = Board::new(4, 5, vec![Player::One,Player::Two]);
        let _ = check_board.try_move(&Player::One, (1, 0));
        let _ = check_board.try_move(&Player::One, (0, 1));

        assert_eq!(board, check_board);
    }

    #[test]
    fn chain_reaction() {
        SimpleLogger::new()
            .init()
            .unwrap();
        let mut board = Board::new(4, 5, vec![Player::One,Player::Two]);

        let cell_pos = board.grid.get(2, 0)
            .unwrap()
            .position;
        assert_eq!(cell_pos, CellPos::Edge);

        board.try_move(&Player::One, (1, 1));
        board.try_move(&Player::One, (1, 1));
        board.try_move(&Player::One, (1, 0));
        board.try_move(&Player::One, (1, 0));
        board.try_move(&Player::One, (0, 1));
        board.try_move(&Player::One, (0, 1));
        board.try_move(&Player::One, (0, 0));
        board.try_move(&Player::One, (0, 0));

        let mut check_board = Board::new(4, 5, vec![Player::One,Player::Two]);
        check_board.try_move(&Player::One, (1, 0));
        check_board.try_move(&Player::One, (1, 0));
        check_board.try_move(&Player::One, (0, 1));
        check_board.try_move(&Player::One, (0, 1));
        check_board.try_move(&Player::One, (0, 2));
        check_board.try_move(&Player::One, (2, 0));
        check_board.try_move(&Player::One, (2, 1));
        check_board.try_move(&Player::One, (1, 2));

        assert_eq!(board, check_board, "\n{}", board);
    }

    #[test]
    fn make_board() {
        SimpleLogger::new()
            .init()
            .unwrap();

        let mut board = Board::new(4, 5, vec![Player::One]);

        board.try_move(&Player::One, (1, 0));
        board.try_move(&Player::One, (1, 0));
        board.try_move(&Player::One, (2, 0));
        board.try_move(&Player::One, (2, 0));

        assert_eq!(
            format!("{}", board),
            r#"╭───┬───┬───┬───┬───╮
│   │   │   │   │   │
├───┼───┼───┼───┼───┤
│ 2 │   │   │   │   │
├───┼───┼───┼───┼───┤
│ 2 │   │   │   │   │
├───┼───┼───┼───┼───┤
│   │   │   │   │   │
╰───┴───┴───┴───┴───╯"#,
            "\n{}", board
        );
    }

    #[test]
    fn losing() {
        SimpleLogger::new()
            .init()
            .unwrap();

        let mut board = Board::new(4, 5, vec![Player::One,Player::Two,Player::Three]);

        board.try_move(&Player::One, (2, 0));
        board.try_move(&Player::Two, (1, 0));

        assert_eq!(board.has_lost(), vec![Player::Three]);

        board.try_move(&Player::Three, (2, 1));
        assert_eq!(board.has_lost(), Vec::<Player>::new());
    }
}
