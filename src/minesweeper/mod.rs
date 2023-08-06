pub mod cell;
pub mod board;

use board::Board;
use cell::Cell;
use rand::Rng;
// use getrandom::getrandom;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(height: usize, width: usize, num_mines: usize) -> Self {
        Self {
            board: Board::new(height, width, num_mines),
        }
    }

    pub fn start_board(&mut self){
        // TODO: make a better implementation
        let mut added_mines = 0;

        let mut rng = rand::thread_rng();

        while added_mines < self.board.get_num_mines() {
            let pos = (rng.gen_range(0..self.board.get_height()), rng.gen_range(0..self.board.get_width()));
            if !self.board.is_mine(pos) {
                self.board.set_mine(pos, true);
                added_mines += 1;
            }
        }

        // Set values
        for i in 0..self.board.get_height() {
            for j in 0..self.board.get_width() {
                self.board.set_value((i, j), self.board.calculate_value((i, j)))             
            }
        }
    }

    pub fn show(&mut self, pos: (usize, usize)) {
        self.board.show_cell(pos);
        if self.board.get_value(pos) == 0 {
            for i in -1..=1 {
                for j in -1..=1 {
                    if pos.0 as isize + i >= 0 && 
                        pos.0 as isize + i < self.get_height() as isize &&
                        pos.1 as isize + j >= 0 &&
                        pos.1 as isize + j < self.get_width() as isize &&
                        self.board.get_cell(((pos.0 as isize + i) as usize , (pos.1 as isize + j) as usize)).is_hidden() {
                        self.show(((pos.0 as isize + i) as usize , (pos.1 as isize + j) as usize))
                    }
                }
            }
        }
    }

    pub fn get_height(&self) -> usize {
        return self.board.get_height();
    }

    pub fn get_width(&self) -> usize {
        return self.board.get_width();
    }   

    pub fn get_board(&self) -> &Board {
        return &self.board;
    }

    pub fn get_cell(&self,pos: (usize, usize)) -> Cell {
        self.board.get_cell(pos)
    }
}