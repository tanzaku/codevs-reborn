
use super::action;
use super::board;

const W: i32 = 10;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Player {
    pub board: board::Board,
    pub obstacle: i32,
    pub skill_guage: i32,
    pub decrease_skill_guage: i32,
}

impl Player {
    pub fn new(board: board::Board, obstacle: i32, skill_guage: i32) -> Self {
        Self { board, obstacle, skill_guage, decrease_skill_guage: 0, }
    }

    pub fn put(&mut self, pack: &[[u8; 2]; 2], action: &action::Action) -> action::ActionResult {
        if self.obstacle >= W {
            self.obstacle -= W;
            self.board.fall_obstacle();
        }
        
        let result = match action {
            action::Action::PutBlock { pos, rot } => self.board.put(pack, *pos, *rot),
            action::Action::UseSkill => self.board.use_skill(),
        };
        self.obstacle -= result.obstacle;
        if result.chains > 0 {
            self.skill_guage += 8;
        }
        self.decrease_skill_guage += result.skill_guage;
        result
    }

    pub fn can_use_skill(&self) -> bool {
        self.skill_guage >= 80
    }
}
