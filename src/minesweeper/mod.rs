pub mod board;
pub mod cell;

use board::Board;
use cell::Cell;
use rand::Rng;
// use getrandom::getrandom;

// use log::info;
// use wasm_bindgen::JsValue;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(height: usize, width: usize, num_mines: usize) -> Self {
        Self {
            board: Board::new(height, width, num_mines),
        }
    }

    pub fn start_board(&mut self) {
        // TODO: make a better implementation
        let mut added_mines = 0;

        let mut rng = rand::thread_rng();

        while added_mines < self.board.get_num_mines() {
            let pos = (
                rng.gen_range(0..self.board.get_height()),
                rng.gen_range(0..self.board.get_width()),
            );
            if !self.board.is_mine(pos) {
                self.board.set_mine(pos, true);
                added_mines += 1;
            }
        }

        for i in 0..self.board.get_height() {
            for j in 0..self.board.get_width() {
                self.board
                    .set_value((i, j), self.board.calculate_value((i, j)))
            }
        }
    }

    pub fn show(&mut self, init_pos: (usize, usize)) {
        if self.board.get_cell(init_pos).get_value() != 0 {
            self.board.show_cell(init_pos);
            return;
        }

        // let mut cells_to_show = Vec::<(usize, usize)>::new();
        let mut cells_to_show = Vec::<(usize, usize)>::from([init_pos]);

        // cells_to_show.push(init_pos);

        self.board.get_cell(init_pos).set_delay(0.0);

        let mut added_cells = 1;

        while added_cells > 0 {
            let new_cells = cells_to_show.len() - added_cells;
            added_cells = 0;

            cells_to_show = cells_to_show[new_cells..cells_to_show.len()].to_vec();

            for k in 0..cells_to_show.len() {
                let pos = cells_to_show[k];
                for i in -1..=1 {
                    for j in -1..=1 {
                        let new_pos =
                            ((pos.0 as isize + i) as usize, (pos.1 as isize + j) as usize);
                        if pos.0 as isize + i < 0
                            || pos.0 as isize + i >= self.get_height() as isize
                            || pos.1 as isize + j < 0
                            || pos.1 as isize + j >= self.get_width() as isize
                            || !self.board.get_cell(new_pos).is_hidden()
                        {
                            continue;
                        }
                        if self.board.get_cell(new_pos).get_value() == 0 {
                            cells_to_show.push(new_pos);
                            added_cells += 1;
                        }

                        // let delay = f32::sqrt(((init_pos.0 as isize - new_pos.0 as isize).pow(2) + (init_pos.1 as isize - new_pos.1 as isize).pow(2)) as f32) * 0.05;

                        let delay = self.board.get_cell(pos).get_delay() + 0.05;

                        self.board.set_delay(new_pos, delay);
                        self.board.show_cell(new_pos);
                    }
                }
            }
        }
    }

    pub fn get_height(&self) -> usize {
        self.board.get_height()
    }

    pub fn get_width(&self) -> usize {
        self.board.get_width()
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_cell(&self, pos: (usize, usize)) -> Cell {
        self.board.get_cell(pos)
    }

    pub fn is_flagged(&self, pos: (usize, usize)) -> bool {
        self.board.get_cell(pos).is_flagged()
    }

    pub fn set_flag(&mut self, pos: (usize, usize), flag: bool) {
        self.board.set_flag(pos, flag);
    }
}
