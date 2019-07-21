// Implementation for a single tac-toe board

use crate::math::random_usize;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellValue {
    O,
    X,
    Empty,
}

impl CellValue {
    pub fn is_empty(&self) -> bool {
        match *self {
            CellValue::Empty => true,
            _ => false,
        }
    }

    pub fn next(&self) -> CellValue {
        match *self {
            CellValue::O => CellValue::X,
            CellValue::X => CellValue::O,
            CellValue::Empty => CellValue::Empty,
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
                return c.clone();
            }
        }

        // Check if we can prevent the opponent from winning
        let opponent = self.current_player.next();

        for c in &free_cells {
            if self.is_winning(c.0, c.1, opponent) {
                return c.clone();
            }
        }

        // Otherwise, make a random move
        free_cells[random_usize(0..free_cells.len())]
    }

    pub fn get_free_cells(&self) -> Vec<(usize, usize)> {
        let mut free_cells = Vec::new();

        for x in 0..3 {
            for y in 0..3 {
                if self.cells[x][y].is_empty() {
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
        self.current_player.is_empty()
    }

    #[rustfmt::skip]
    pub fn is_winning(&self, x: usize, y: usize, p: CellValue) -> bool {
        let m = Mask::from(self, p);
        match (x, y) {
            (0, 0) => (
                (m.c(0, 1) && m.c(0, 2)) ||
                (m.c(1, 0) && m.c(2, 0)) ||
                (m.c(1, 1) && m.c(2, 2))
            ),
            (0, 1) => (m.c(0, 0) && m.c(0, 2)) || (m.c(1, 1) && m.c(2, 1)),
            (0, 2) => (
                (m.c(0, 0) && m.c(0, 1)) ||
                (m.c(1, 2) && m.c(2, 2)) ||
                (m.c(2, 0) && m.c(1, 1))
            ),
            (1, 0) => (m.c(1, 1) && m.c(1, 2)) || (m.c(0, 0) && m.c(2, 0)),
            (1, 1) => (
                (m.c(0, 0) && m.c(2, 2)) ||
                (m.c(0, 2) && m.c(2, 0)) ||
                (m.c(0, 1) && m.c(2, 1)) ||
                (m.c(1, 0) && m.c(1, 2))
            ),
            (1, 2) => (m.c(1, 0) && m.c(1, 1)) || (m.c(0, 2) && m.c(2, 2)),
            (2, 0) => (
                (m.c(2, 1) && m.c(2, 2)) ||
                (m.c(0, 0) && m.c(1, 0)) ||
                (m.c(1, 1) && m.c(0, 2))
            ),
            (2, 1) => (m.c(2, 0) && m.c(2, 2)) || (m.c(0, 1) && m.c(1, 1)),
            (2, 2) => (
                (m.c(2, 0) && m.c(2, 1)) ||
                (m.c(0, 2) && m.c(1, 2)) ||
                (m.c(0, 0) && m.c(1, 1))
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
                self.current_player.next()
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

// An utility to type less
struct Mask<'b> {
    cells: &'b [[CellValue; 3]; 3],
    player: CellValue,
}

impl<'b> Mask<'b> {
    fn from(board: &'b Board, player: CellValue) -> Self {
        Mask {
            cells: &board.cells,
            player,
        }
    }
    fn c(&self, x: usize, y: usize) -> bool {
        self.cells[x][y] == self.player
    }
}
