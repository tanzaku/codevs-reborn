
use super::action;
use super::board;
use super::rand;

use std::collections::VecDeque;

const DEAD_LINE_Y: i32 = 16;
const W: i32 = 10;

#[derive(Clone, Default)]
struct BeamState {
    board: board::Board,
    score: i32,
    actions: Vec<u8>,
}

impl BeamState {
    fn new(board: board::Board, score: i32, actions: Vec<u8>) -> Self {
        Self { board, score, actions }
    }
}

pub struct RensaPlan {
    rand: rand::XorShiftL,
    packs: Vec<[[u8; 2]; 2]>,
    replay: VecDeque<action::Action>
}

impl RensaPlan {
    pub fn new() -> Self {
        Self {
            rand: rand::XorShiftL::new(),
            packs: Vec::new(),
            replay: VecDeque::new(),
        }
    }

    pub fn set_pack(&mut self, packs: Vec<[[u8; 2]; 2]>) {
        self.packs = packs;
    }

    fn is_dangerous(board: &board::Board) -> bool {
        board.max_height() as i32 >= DEAD_LINE_Y - 1
    }
    
    pub fn calc_rensa_plan(&mut self, cur_turn: usize, board: &board::Board, obstacle_stock: i32) {
        let mut next = Vec::new();
        let mut cur = vec![BeamState::new(board.clone(), 0, Vec::new())];
        let fire_turn = 20;
        let beam_width = 100 * 3;
        let actions = action::Action::all_actions();
        let allow_dead_line = Self::is_dangerous(board);
        let fall = self.packs[cur_turn + fire_turn].clone();
        for i in 0..fire_turn {
            let turn = cur_turn + i;
            let pack = self.packs[turn].clone();
            let rest_obstacle = obstacle_stock - i as i32 * W;
            cur.iter().for_each(|b| {
                actions.iter().for_each(|a| {
                    if let action::Action::PutBlock { pos, rot } = *a {
                        let mut board = b.board.clone();
                        if rest_obstacle >= W {
                            board.fall_obstacle();
                        }
                        board.put(&pack, pos, rot);
                        if board.is_dead() {
                            return;
                        }

                        // obstacleが降ってくると危ないので
                        if !allow_dead_line && Self::is_dangerous(&board) {
                            return;
                        }
                        let mut rensa_eval_board = board.clone();
                        let rensa = rensa_eval_board.put(&fall, 0, 0) as i32;
                        let score = rensa * 10000 + (self.rand.next() & 0xF) as i32 - board.max_height() as i32;
                        let mut actions = b.actions.clone();
                        actions.push(a.into());
                        next.push(BeamState::new(board, score, actions));
                    }
                });
            });
            next.sort_by(|a,b| b.score.cmp(&a.score));
            next.resize(beam_width, Default::default());
            std::mem::swap(&mut cur, &mut next);
            // eprintln!("search: {} {}", i, cur[0].score);
            next.clear();
        }
        eprintln!("best: {} {} {} {}", cur_turn, cur[0].score, cur[0].actions.len(), obstacle_stock);
        let best = &cur[0].actions;
        self.replay = best.iter().map(|s| action::Action::from(*s)).collect();
        self.replay.push_back(action::Action::PutBlock{ pos: 0, rot: 0, })
        // for _ in 0..100 {
        //     self.replay.push_back(action::Action::PutBlock{ pos: 0, rot: 0, })
        // }
    }

    pub fn replay(&mut self) -> action::Action {
        // let pos = (self.rand.next() % 9) as usize;
        // let rot = (self.rand.next() % 4) as usize;
        // action::Action::PutBlock{ pos, rot }
        self.replay.pop_front().unwrap()
    }

    pub fn exists(&self) -> bool {
        !self.replay.is_empty()
    }
}


