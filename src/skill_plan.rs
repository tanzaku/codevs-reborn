
use super::action;
use super::board;
use super::player;
use super::rand;

use std::collections::VecDeque;
use std::cmp::Ordering;

const DEAD_LINE_Y: i32 = 16;
const W: i32 = 10;

#[derive(Clone, Default, PartialEq, Eq)]
struct BeamState {
    player: player::Player,
    score: i32,
    actions: Vec<u8>,
}

impl BeamState {
    fn new(player: player::Player, score: i32, actions: Vec<u8>) -> Self {
        Self { player, score, actions }
    }
}

impl Ord for BeamState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for BeamState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct SkillPlan {
    rand: rand::XorShiftL,
    packs: Vec<[[u8; 2]; 2]>,
    replay: VecDeque<action::Action>
}

impl SkillPlan {
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
    
    pub fn calc_skill_plan(&mut self, cur_turn: usize, player: &player::Player) {
        let mut next = Vec::new();
        let mut cur = vec![BeamState::new(player.clone(), 0, Vec::new())];
        let beam_width = 100 * 3 * 3;
        let actions = action::Action::all_actions();

        let fire_action = action::Action::UseSkill;
        let fire_turn = 10;
        let fall = self.packs[cur_turn + fire_turn].clone();

        for i in 0..fire_turn {
            let turn = cur_turn + i;
            let pack = self.packs[turn].clone();
            cur.iter().for_each(|b| {
                actions.iter().for_each(|a| {
                    if let action::Action::UseSkill = a {
                        if !b.player.can_use_skill() {
                            return;
                        }
                    }

                    let mut player = b.player.clone();
                    player.put(&pack, a);
                    if player.board.is_dead() {
                        return;
                    }

                    let mut rensa_eval_board = player.clone();
                    let result = rensa_eval_board.put(&fall, &fire_action);
                    let skill_guage_score = if player.can_use_skill() {
                                                100000000
                                            } else {
                                                player.skill_guage * 10000
                                            };
                    let score = -(
                                    // maximize
                                    result.obstacle * 1000
                                    + skill_guage_score
                                    - player.board.max_height() as i32 * 10
                                    + (self.rand.next() & 0xF) as i32
                                );
                    let mut actions = b.actions.clone();
                    actions.push(a.into());
                    next.push(BeamState::new(player, score, actions));
                });
            });
            next.sort();
            next.resize(beam_width, Default::default());
            std::mem::swap(&mut cur, &mut next);
            next.clear();
            // if cur[0].player.can_use_skill() {
            //     break;
            // }
            // if cur[0].player.obstacle <= -W {
            //     break;
            // }
        }
        eprintln!("best: {} {} {} {}", cur_turn, cur[0].player.obstacle, cur[0].actions.len(), cur[0].player.skill_guage);
        let best = &cur[0].actions;
        self.replay = best.iter().map(|s| action::Action::from(*s)).collect();
        self.replay.push_back(fire_action)
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

    pub fn can_replay(&self, player: &player::Player) -> bool {
        if let Some(action::Action::UseSkill) = self.replay.front() {
            player.can_use_skill()
        } else {
            self.exists()
        }
    }
}


