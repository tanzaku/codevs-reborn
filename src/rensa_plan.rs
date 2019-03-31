
use super::action;
use super::board;
use super::rand;

pub struct RensaPlan {
    rand: rand::XorShiftL,
}

impl RensaPlan {
    pub fn new() -> Self {
        Self {
            rand: rand::XorShiftL::new(),
        }
    }
    
    pub fn calc_rensa_plan(&mut self, cur_turn: usize, board: &board::Board) {
        // let mut board = board.clone();

    }

    pub fn replay(&mut self) -> action::Action {
        let pos = (self.rand.next() % 9) as usize;
        let rot = (self.rand.next() % 4) as usize;
        action::Action::PutBlock{ pos, rot }
    }

    pub fn exists(&self) -> bool {
        false
    }
}


