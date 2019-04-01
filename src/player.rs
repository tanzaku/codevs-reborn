
use super::action;
use super::board;

pub struct Player {
    board: board::Board,
    obstacle_stock: i32,
    skill_guage: i32,
}

impl Player {
    fn new(board: board::Board, obstacle_stock: i32, skill_guage: i32) -> Self {
        Self { board, obstacle_stock, skill_guage }
    }

    fn put(&mut self, pack: [[u8; 2]; 2], action: action::Action) -> ActionResult {
        match action {
            action::Action::PutBlock { pos, rot } => self.board.put(pack, pos, rot),
            action::Action::UseSkill => self.use_skill(),
        };
    }
}
