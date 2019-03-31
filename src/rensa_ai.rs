

// Wの値が紛らわしい
const W: i32 = 10;
const H: i32 = 16;

use std::str::FromStr;
use std::io::Read;
use std::io::StdinLock;

use super::action;
use super::board;
use super::rensa_plan;

const MAX_TURN: usize = 500;

pub struct RensaAi<'a> {
    cur_turn: usize,
    stdin_lock: StdinLock<'a>,
    packs: Vec<[[u8; 2]; 2]>,
    board: board::Board,
    rensa_plan: rensa_plan::RensaPlan,
}

impl<'a> RensaAi<'a> {
    pub fn new(lock: StdinLock<'a>) -> Self {
        Self {
            cur_turn: 0,
            stdin_lock: lock,
            packs: Vec::new(),
            board: board::Board::new(),
            rensa_plan: rensa_plan::RensaPlan::new(),
        }
    }

    fn read1<T: FromStr>(&mut self) -> T {
        let token = self.stdin_lock.by_ref().bytes().map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        token.parse::<T>().ok().unwrap()
    }

    fn read_game_input(&mut self) {
        (0..MAX_TURN).for_each(|_| {
            let v1 = self.read1();
            let v2 = self.read1();
            let v3 = self.read1();
            let v4 = self.read1();
            self.read1::<String>();
            self.packs.push([[v1, v2], [v3, v4]]);
        });
        self.rensa_plan.set_pack(self.packs.clone());
    }

    fn read_turn_input(&mut self) {
        self.cur_turn = self.read1();
        // eprintln!("start read {}", self.cur_turn);
        let rest_time_in_milli = self.read1::<u32>();
        let obstacle_stock = self.read1::<u32>();
        let skill_guage = self.read1::<u32>();
        (0..W*H).for_each(|_| { self.read1::<u8>(); });
        self.read1::<String>();

        let rest_time_in_milli = self.read1::<u32>();
        let obstacle_stock = self.read1::<u32>();
        let skill_guage = self.read1::<u32>();
        (0..W*H).for_each(|_| { self.read1::<u8>(); });
        self.read1::<String>();
    }

    pub fn exec(&mut self) {
        println!("test-rensa-ai");
        self.read_game_input();
        loop {
            self.read_turn_input();
            let act = self.think();

            if let action::Action::PutBlock { pos, rot, } = act {
                self.board.put(&self.packs[self.cur_turn], pos, rot);
                // eprintln!("turn={}", self.cur_turn);
                // eprintln!("{:?}", self.board);
            }
            println!("{}", act);
        }
        // while let Some(act) = self.think() {
        //     println!("{}", act);
        // }
    }

    fn think(&mut self) -> action::Action {
        if !self.rensa_plan.exists() {
            self.rensa_plan.calc_rensa_plan(self.cur_turn, &self.board);
        }
        self.rensa_plan.replay()
    }
}
