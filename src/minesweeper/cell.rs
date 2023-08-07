#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pos: (usize, usize), // (height, width)
    mine: bool,
    value: usize,
    hidden: bool,
    flagged: bool,
    delay: f32
}

impl Cell {
    pub fn new(pos: (usize, usize)) -> Self {
        return Self {
            pos,
            mine: false,
            value: 0,
            hidden: true,
            flagged: false,
            delay: 0.0
        };
    }

    pub fn get_pos(&self) -> (usize, usize) {
        return self.pos;
    }

    pub fn is_mine(&self) -> bool {
        return self.mine;
    }

    pub fn set_mine(&mut self, new_mine: bool) {
        self.mine = new_mine;
    }

    pub fn get_value(&self) -> usize {
        return self.value;
    }

    pub fn set_value(&mut self, new_value: usize) {
       self.value = new_value;
    }

    pub fn show(&mut self) {
        self.hidden = false;
        self.flagged = false;
    }

    pub fn is_hidden(&self) -> bool {
        return self.hidden;
    }

    pub fn is_flagged(&self) -> bool {
        return self.flagged;
    }

    pub fn set_flag(&mut self, new_flag: bool) {
        if !self.hidden {
            self.flagged = false;
            return;
        }
        self.flagged = new_flag;
    }

    pub fn get_delay(&self) -> f32 {
        self.delay
    }

    pub fn set_delay(&mut self, delay: f32) {
        self.delay = delay;
    }

}

impl ToString for Cell {
    fn to_string(&self) -> String {
        if self.hidden {
            return " ".to_string();
        }
        if self.is_mine() {
            return " ".to_string();
        }
        if self.value == 0 {
            return " ".to_string();
        }

        return self.value.to_string();
    }
}