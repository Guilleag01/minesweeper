
use super::cell::Cell;

#[derive(Clone, PartialEq)]
pub struct Board {
    board: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
    num_mines: usize    
}

impl Board {
    pub fn new(height: usize, width: usize, num_mines: usize) -> Self {
        let mut t: Vec<Vec<Cell>> = Vec::new();
        for i in 0..height {
            t.push(Vec::new());
            for j in 0..width {
                t[i].push(Cell::new((i, j)))
            }
        }

        return Self {
            board: t,
            height,
            width,
            num_mines
        };
    }

    pub fn calculate_value(&self, pos: (usize, usize)) -> usize {
        let mut value: usize = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                let new_pos: (isize, isize) = ((pos.0 as isize) + i, (pos.1 as isize) + j);
                if new_pos.0 >= 0 && new_pos.0 < (self.height as isize) && new_pos.1 >= 0 && new_pos.1 < (self.width as isize) {
                    value += self.is_mine((new_pos.0 as usize, new_pos.1 as usize)) as usize;
                }
            }
        }
        return value;
    }

    pub fn get_board(&self) -> &Vec<Vec<Cell>> {
        return &self.board;
    }

    pub fn set_mine(&mut self, pos: (usize, usize), value: bool) {
        self.board[pos.0][pos.1].set_mine(value);
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_num_mines(&self) -> usize {
        return self.num_mines;
    }

    pub fn is_mine(&self, pos: (usize, usize)) -> bool {
        return self.board[pos.0][pos.1].is_mine();
    }

    pub fn get_value(&self, pos: (usize, usize)) -> usize {
        return self.board[pos.0][pos.1].get_value();
    }

    pub fn get_cell(&self, pos: (usize, usize)) -> Cell {
        return self.board[pos.0][pos.1];
    }

    pub fn set_value(&mut self, pos: (usize, usize), new_value: usize) {
        return self.board[pos.0][pos.1].set_value(new_value)
    }

    pub fn show_cell(&mut self, pos: (usize, usize)) {
        self.board[pos.0][pos.1].show();
    }

}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str("  ");
        for i in 0..self.width {
            // result.push_str(" ");
            result.push_str(i.to_string().as_str());
        }
        result.push_str("\n");

        for i in 0..self.height {
            result.push_str(i.to_string().as_str());
            result.push_str(" ");

            for j in 0..self.width {
                result.push_str(self.get_cell((i, j)).to_string().as_str());
            }
            result.push_str("\n");
        }

        return result;
    }
}