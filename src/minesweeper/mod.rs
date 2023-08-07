pub mod cell;
pub mod board;

use board::Board;
use cell::Cell;
use rand::Rng;
// use getrandom::getrandom;

use log::info;
use wasm_bindgen::JsValue;

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

    pub fn show(&mut self, init_pos: (usize, usize)) {
        if self.board.get_cell(init_pos).get_value() != 0 {
            self.board.show_cell(init_pos);
            return;
        }

        let mut cells_to_show = Vec::<(usize, usize)>::new();
        
        cells_to_show.push(init_pos);

        let mut added_cells = 1;
        
        while added_cells > 0 {
            // info!("{:?}", cells_to_show.len() - added_cells);
            let new_cells = cells_to_show.len() - added_cells;
            added_cells = 0;
            
            cells_to_show = cells_to_show[new_cells..cells_to_show.len()].to_vec();

            for k in 0..cells_to_show.len() {
                let pos = cells_to_show[k];
                for i in -1..=1 {
                    for j in -1..=1 {
                        let new_pos = ((pos.0 as isize + i) as usize, (pos.1 as isize + j) as usize);
                        if pos.0 as isize + i < 0 ||
                            pos.0 as isize + i >= self.get_height() as isize ||
                            pos.1 as isize + j < 0 ||
                            pos.1 as isize + j >= self.get_width() as isize ||
                            !self.board.get_cell(new_pos).is_hidden() {
                            continue;
                        }
                        if self.board.get_cell(new_pos).get_value() == 0 {
                            cells_to_show.push(new_pos);
                            added_cells += 1;
                        }
                        self.board.set_delay(new_pos, f32::sqrt(((init_pos.0 as isize - new_pos.0 as isize).pow(2) + (init_pos.1 as isize - new_pos.1 as isize).pow(2)) as f32) * 0.05);
                        self.board.show_cell(new_pos);
                    }
                }
            }   
        }

        // for pos in cells_to_show {
        //     self.board.show_cell(pos);
        // }

        
        

        // self.board.show_cell(pos);
        // if self.board.get_value(pos) == 0 {
        //     for i in -1..=1 {
        //         for j in -1..=1 {
        //             if pos.0 as isize + i >= 0 && 
        //                 pos.0 as isize + i < self.get_height() as isize &&
        //                 pos.1 as isize + j >= 0 &&
        //                 pos.1 as isize + j < self.get_width() as isize &&
        //                 self.board.get_cell(((pos.0 as isize + i) as usize , (pos.1 as isize + j) as usize)).is_hidden() {
        //                 self.show(((pos.0 as isize + i) as usize , (pos.1 as isize + j) as usize))
        //             }
        //         }
        //     }
        // }
        

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

    pub fn get_cell(&self, pos: (usize, usize)) -> Cell {
        self.board.get_cell(pos)
    }
}