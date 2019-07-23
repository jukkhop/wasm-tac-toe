// Implementation for a single tac-toe board

use crate::math::random_usize;

#[derive(Clone, Copy, PartialEq)]
pub enum CellValue {
    O,
    X,
    Empty,
}

impl CellValue {
    pub fn is_empty(value: CellValue) -> bool {
        match value {
            CellValue::Empty => true,
            _ => false,
        }
    }

    pub fn next(value: CellValue) -> CellValue {
        match value {
            CellValue::O => CellValue::X,
            CellValue::X => CellValue::O,
            CellValue::Empty => CellValue::Empty,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pub cells: [[CellValue; 3]; 3],
    pub cross_progress: f64,
    pub current_player: CellValue,
    pub is_crossed: bool,
    pub move_count: usize,
    pub progress: [[f64; 3]; 3],
    pub winner: CellValue,
}

impl Board {
    fn find_move(&mut self) -> (usize, usize) {
        let free_cells = self.get_free_cells();

        // Check if there's a winning move
        for c in &free_cells {
            if self.is_winning(c.0, c.1, self.current_player) {
                return *c;
            }
        }

        // Check if we can prevent the opponent from winning
        let opponent = CellValue::next(self.current_player);

        for c in &free_cells {
            if self.is_winning(c.0, c.1, opponent) {
                return *c;
            }
        }

        // Otherwise, make a random move
        free_cells[random_usize(0..free_cells.len())]
    }

    pub fn get_free_cells(&self) -> Vec<(usize, usize)> {
        let mut free_cells = Vec::new();

        for x in 0..3 {
            for y in 0..3 {
                if CellValue::is_empty(self.cells[x][y]) {
                    free_cells.push((x, y));
                }
            }
        }

        free_cells
    }

    #[rustfmt::skip]
    #[allow(dead_code)]
    pub fn get_win_row(&self) -> Option<(i32, i32, i32, i32)> {
        let w = self.winner;
        let c = self.cells;

        if c[0][0] == w && c[0][1] == w && c[0][2] == w {
           return Some((0, 0, 0, 2));
        }

        if c[1][0] == w && c[1][1] == w && c[1][2] == w {
           return Some((1, 0, 1, 2));
        }

        if c[2][0] == w && c[2][1] == w && c[2][2] == w {
           return Some((2, 0, 2, 2));
        }

        if c[0][0] == w && c[1][0] == w && c[2][0] == w {
           return Some((0, 0, 2, 0));
        }

        if c[0][1] == w && c[1][1] == w && c[2][1] == w {
           return Some((0, 1, 2, 1));
        }

        if c[0][2] == w && c[1][2] == w && c[2][2] == w {
           return Some((0, 2, 2, 2));
        }

        if c[0][0] == w && c[1][1] == w && c[2][2] == w {
           return Some((0, 0, 2, 2));
        }

        if c[0][2] == w && c[1][1] == w && c[2][0] == w {
           return Some((0, 2, 2, 0));
        }

        None
    }

    pub fn is_finished(&self) -> bool {
        CellValue::is_empty(self.current_player)
    }

    #[rustfmt::skip]
    pub fn is_winning(&self, x: usize, y: usize, p: CellValue) -> bool {
        let c = self.cells;
        match (x, y) {
            (0, 0) => (
                (c[0][1] == p && c[0][2] == p) ||
                (c[1][0] == p && c[2][0] == p) ||
                (c[1][1] == p && c[2][2] == p)
            ),
            (0, 1) => (c[0][0] == p && c[0][2] == p) || (c[1][1] == p && c[2][1] == p),
            (0, 2) => (
                (c[0][0] == p && c[0][1] == p) ||
                (c[1][2] == p && c[2][2] == p) ||
                (c[2][0] == p && c[1][1] == p)
            ),
            (1, 0) => (c[1][1] == p && c[1][2] == p) || (c[0][0] == p && c[2][0] == p),
            (1, 1) => (
                (c[0][0] == p && c[2][2] == p) ||
                (c[0][2] == p && c[2][0] == p) ||
                (c[0][1] == p && c[2][1] == p) ||
                (c[1][0] == p && c[1][2] == p)
            ),
            (1, 2) => (c[1][0] == p && c[1][1] == p) || (c[0][2] == p && c[2][2] == p),
            (2, 0) => (
                (c[2][1] == p && c[2][2] == p) ||
                (c[0][0] == p && c[1][0] == p) ||
                (c[1][1] == p && c[0][2] == p)
            ),
            (2, 1) => (c[2][0] == p && c[2][2] == p) || (c[0][1] == p && c[1][1] == p),
            (2, 2) => (
                (c[2][0] == p && c[2][1] == p) ||
                (c[0][2] == p && c[1][2] == p) ||
                (c[0][0] == p && c[1][1] == p)
            ),
            (_, _) => { unreachable!() },
        }
    }

    pub fn play(&mut self) {
        let (x, y) = self.find_move();
        let p = self.current_player;

        self.move_count += 1;
        self.cells[x][y] = p;

        if self.is_winning(x, y, p) {
            self.winner = p;
            self.current_player = CellValue::Empty;
        } else {
            self.current_player = if self.move_count == 9 {
                CellValue::Empty
            } else {
                CellValue::next(self.current_player)
            };
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[CellValue::Empty; 3]; 3],
            cross_progress: 0.0,
            current_player: CellValue::O,
            is_crossed: false,
            move_count: 0,
            progress: [[0.0; 3]; 3],
            winner: CellValue::Empty,
        }
    }
}
