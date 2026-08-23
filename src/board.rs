use array2d::Array2D;

#[derive(Copy,Clone,PartialEq,Debug)]
enum CellPos {
    Corner,
    Edge,
    Bulk,
}

#[derive(Copy,Clone)]
enum Player {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Copy,Clone)]
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
}

struct Board {
    grid: Array2D<Cell>,
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
}
