
use super::action;
use super::board;
use super::rand;

use std::collections::VecDeque;

const DEAD_LINE_Y: i32 = 16;

#[derive(Clone, Default)]
struct BeamState {
    board: board::Board,
    score: u32,
    actions: Vec<u8>,
}

impl BeamState {
    fn new(board: board::Board, score: u32, actions: Vec<u8>) -> Self {
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
    
    pub fn calc_rensa_plan(&mut self, cur_turn: usize, board: &board::Board) {
        let mut next = Vec::new();
        let mut cur = vec![BeamState::new(board.clone(), 0, Vec::new())];
        let fire_turn = 15;
        let beam_width = 100;
        let actions = action::Action::all_actions();
        for i in 0..fire_turn {
            let turn = cur_turn + i;
            let pack = self.packs[turn].clone();
            let fall = self.packs[cur_turn + fire_turn].clone();
            cur.iter().for_each(|b| {
                actions.iter().for_each(|a| {
                    if let action::Action::PutBlock { pos, rot } = *a {
                        let mut board = b.board.clone();
                        board.put(&pack, pos, rot);
                        if board.is_dead() {
                            return;
                        }

                        // obstacleが降ってくると危ないので
                        if board.max_height() as i32 >= DEAD_LINE_Y - 1 {
                            return;
                        }
                        let mut rensa_eval_board = board.clone();
                        let rensa = rensa_eval_board.put(&fall, 0, 0) as u32;
                        let score = rensa * 10000 + (self.rand.next() & 0xF) as u32;
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
        eprintln!("best: {}", cur[0].score);
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


