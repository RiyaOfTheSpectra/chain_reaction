#[allow(unused)]
use std::option::Option;

use array2d::Array2D;

type Location = (usize, usize);

#[derive(Copy,Clone,Debug,PartialEq)]
enum Error {
    CellOccupied,
    LocationInvalid,
}

#[derive(Copy,Clone,Debug,PartialEq)]
enum CellPos {
    Corner,
    Edge,
    Bulk,
}

#[derive(Copy,Clone,Debug,PartialEq)]
enum Player {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Copy,Clone,Debug)]
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
        match self.position {
            CellPos::Corner => self.contents > 1u8,
            CellPos::Edge => self.contents > 2u8,
            CellPos::Bulk => self.contents > 3u8,
        }
    }

    fn increment(&mut self) {
        self.contents += 1;
    }

    fn set_player(&mut self, player: &Player) {
        self.player = *player;
    }

    fn can_move_player(&self, player: &Player) -> bool {
        (self.contents == 0) || (self.player == *player)
    }
}

struct Board {
    grid: Array2D<Cell>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn new(rows: usize, cols: usize) -> Self {
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
        }
    }

    fn get_neighbours(&self, location: Location) -> Option<Vec<Location>> {
        let row_lim = self.rows;
        let col_lim = self.cols;

        let (row, col) = location;

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

    fn try_move(&mut self, player: &Player, location: Location) -> Result<bool, Error> {
        let (row, col) = location;
        match self.grid.get(row, col) {
            Some(cell) => if !cell.can_move_player(player) {
                Err(Error::CellOccupied)
            } else {
                let mut new_cell = cell.clone();
                new_cell.increment();
                let burst = new_cell.is_bursting();
                let _ = self.grid.set(row, col, new_cell);
                Ok(burst)
            }
            None => Err(Error::LocationInvalid)
        }
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

        let board = Board::new(4, 5);

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
        let board = Board::new(6, 8);

        assert_eq!(board.get_neighbours((0, 0)), Some(vec![
            (0, 1),
            (1, 0),
        ]));

        assert_eq!(board.get_neighbours((5, 0)), Some(vec![
            (4, 0),
            (5, 1),
        ]));

        assert_eq!(board.get_neighbours((5, 7)), Some(vec![
            (4, 7),
            (5, 6),
        ]));

        assert_eq!(board.get_neighbours((0, 7)), Some(vec![
            (0, 6),
            (1, 7),
        ]));

        assert_eq!(board.get_neighbours((0, 1)), Some(vec![
            (0, 0),
            (0, 2),
            (1, 1),
        ]));

        assert_eq!(board.get_neighbours((1, 0)), Some(vec![
            (0, 0),
            (1, 1),
            (2, 0),
        ]));

        assert_eq!(board.get_neighbours((5, 1)), Some(vec![
            (4, 1),
            (5, 0),
            (5, 2),
        ]));

        assert_eq!(board.get_neighbours((1, 7)), Some(vec![
            (0, 7),
            (1, 6),
            (2, 7),
        ]));

        assert_eq!(board.get_neighbours((2, 3)), Some(vec![
            (1, 3),
            (2, 2),
            (2, 4),
            (3, 3),
        ]));

        assert_eq!(board.get_neighbours((6, 8)), None);
        assert_eq!(board.get_neighbours((6, 7)), None);
        assert_eq!(board.get_neighbours((5, 8)), None);
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
        let mut board = Board::new(4, 5);

        assert_eq!(board.try_move(&Player::One, (0, 0)), Ok(false));
        assert_eq!(board.try_move(&Player::Two, (0, 0)), Err(Error::CellOccupied));
        assert_eq!(board.try_move(&Player::One, (0, 0)), Ok(true));
        assert_eq!(board.try_move(&Player::One, (4, 3)), Err(Error::LocationInvalid));

    }

    #[test]
    fn make_board() {
        SimpleLogger::new()
            .init()
            .unwrap();

        let _ = Board::new(4, 5);
    }
}
