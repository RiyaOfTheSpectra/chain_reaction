use std::option::Option;

use array2d::Array2D;

#[derive(Copy,Clone,PartialEq,Debug)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    fn new(row: usize, col: usize) -> Self {
        Location {
            row: row,
            col: col,
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
enum CellPos {
    Corner,
    Edge,
    Bulk,
}

#[derive(Copy,Clone,Debug)]
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
            position: position,
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

    fn check_player(&self, player: Player) -> bool {
        match self.player {
            player => true,
            _ => false,
        }
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
            rows,
            cols,
        );

        let _ = grid.set_column_major(0, Cell::new(CellPos::Edge));
        let _ = grid.set_column_major(cols - 1, Cell::new(CellPos::Edge));

        let _ = grid.set_row_major(0, Cell::new(CellPos::Edge));
        let _ = grid.set_row_major(rows - 1, Cell::new(CellPos::Edge));

        let _ = grid.set(0, 0, Cell::new(CellPos::Corner));
        let _ = grid.set(0, cols-1, Cell::new(CellPos::Corner));
        let _ = grid.set(rows-1, 0, Cell::new(CellPos::Corner));
        let _ = grid.set(rows-1, cols-1, Cell::new(CellPos::Corner));

        Board {
            grid: grid,
            rows: rows-1,
            cols: cols-1,
        }
    }

    fn get_neighbours(&self, row: usize, col: usize) -> Option<Vec<Location>> {
        let row_lim = self.rows;
        let col_lim = self.cols;

        if (row > row_lim) || (col > col_lim) {
            return None;
        }

        if (row == 0) {
            if (col == 0) {
                return Some(vec![
                    Location::new(0, 1),
                    Location::new(1, 0),
                ]);
            } else if (col == col_lim) {
                return Some(vec![
                    Location::new(0, col_lim-1),
                    Location::new(1, col_lim),
                ]);
            } else {
                return Some(vec![
                    Location::new(0, col-1),
                    Location::new(0, col+1),
                    Location::new(1, col),
                ]);
            }
        } else if (row == row_lim) {
            if (col == 0) {
                return Some(vec![
                    Location::new(row_lim-1, 0),
                    Location::new(row_lim, 1),
                ]);
            } else if (col == col_lim) {
                return Some(vec![
                    Location::new(row_lim-1, col_lim),
                    Location::new(row_lim, col_lim-1),
                ]);
            } else {
                return Some(vec![
                    Location::new(row_lim-1, col),
                    Location::new(row_lim, col-1),
                    Location::new(row_lim, col+1),
                ]);
            }
        } else {
            if (col == 0) {
                return Some(vec![
                    Location::new(row-1, 0),
                    Location::new(row, 1),
                    Location::new(row+1, 0),
                ]);
            } else if (col == col_lim) {
                return Some(vec![
                    Location::new(row-1, col_lim),
                    Location::new(row, col_lim-1),
                    Location::new(row+1, col_lim),
                ]);
            } else {
                return Some(vec![
                    Location::new(row-1, col),
                    Location::new(row, col-1),
                    Location::new(row, col+1),
                    Location::new(row+1, col),
                ]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let _ = row_vec[0].clone()
            .into_iter()
            .map(|cell| { assert_eq!(cell.position, CellPos::Edge); });
        let _ = row_vec[rows-1].clone()
            .into_iter()
            .map(|cell| { assert_eq!(cell.position, CellPos::Edge); });
        let _ = col_vec[0].clone()
            .into_iter()
            .map(|cell| { assert_eq!(cell.position, CellPos::Edge); });
        let _ = col_vec[cols-1].clone()
            .into_iter()
            .map(|cell| { assert_eq!(cell.position, CellPos::Edge); });
    }

    #[test]
    fn check_neighbours() {
        let board = Board::new(6, 8);

        assert_eq!(board.get_neighbours(0, 0), Some(vec![
            Location::new(0, 1),
            Location::new(1, 0),
        ]));

        assert_eq!(board.get_neighbours(5, 0), Some(vec![
            Location::new(4, 0),
            Location::new(5, 1),
        ]));

        assert_eq!(board.get_neighbours(5, 7), Some(vec![
            Location::new(4, 7),
            Location::new(5, 6),
        ]));

        assert_eq!(board.get_neighbours(0, 7), Some(vec![
            Location::new(0, 6),
            Location::new(1, 7),
        ]));

        assert_eq!(board.get_neighbours(0, 1), Some(vec![
            Location::new(0, 0),
            Location::new(0, 2),
            Location::new(1, 1),
        ]));

        assert_eq!(board.get_neighbours(1, 0), Some(vec![
            Location::new(0, 0),
            Location::new(1, 1),
            Location::new(2, 0),
        ]));

        assert_eq!(board.get_neighbours(5, 1), Some(vec![
            Location::new(4, 1),
            Location::new(5, 0),
            Location::new(5, 2),
        ]));

        assert_eq!(board.get_neighbours(1, 7), Some(vec![
            Location::new(0, 7),
            Location::new(1, 6),
            Location::new(2, 7),
        ]));

        assert_eq!(board.get_neighbours(2, 3), Some(vec![
            Location::new(1, 3),
            Location::new(2, 2),
            Location::new(2, 4),
            Location::new(3, 3),
        ]));

        assert_eq!(board.get_neighbours(6, 8), None);
    }
}
