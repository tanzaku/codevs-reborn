

// Wの値が紛らわしい
const W: i32 = 10;
const H: i32 = 16;

use std::str::FromStr;
use std::io::Read;
use std::io::StdinLock;

use super::action;
use super::board;
use super::player;
use super::rensa_plan;

const MAX_TURN: usize = 500;

pub struct LambdaAi<'a> {
    cur_turn: usize,
    stdin_lock: StdinLock<'a>,
    packs: Vec<[[u8; 2]; 2]>,
    player: player::Player,
    enemy: player::Player,
}

impl<'a> LambdaAi<'a> {
    pub fn new(lock: StdinLock<'a>) -> Self {
        Self {
            cur_turn: 0,
            stdin_lock: lock,
            packs: Vec::new(),
            player: player::Player::new(board::Board::new(), 0, 0),
            enemy: player::Player::new(board::Board::new(), 0, 0),
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

    fn read_board(&mut self) -> board::Board {
        let mut board = [0; (W * H) as usize];
        (0..W*H).for_each(|p| { board[p as usize] = self.read1::<u8>(); });
        board::Board::from_board(board)
    }

    fn read_turn_input(&mut self) {
        self.prev_obstacle_stock = self.player.obstacle;

        self.cur_turn = self.read1();
        // eprintln!("start read {}", self.cur_turn);
        let rest_time_in_milli = self.read1::<u32>();
        self.player.obstacle = self.read1();
        self.player.skill_guage = self.read1();
        self.player.board = self.read_board();
        self.read1::<String>();

        let rest_time_in_milli = self.read1::<u32>();
        self.enemy.obstacle = self.read1();
        self.enemy.skill_guage = self.read1();
        self.enemy.board = self.read_board();
        self.read1::<String>();
    }

    pub fn exec(&mut self) {
        println!("test-lambda-ai");
        self.read_game_input();
        loop {
            self.read_turn_input();
            let act = self.think();

            // eprintln!("turn={}, obs={}, prev_obs={}", self.cur_turn, self.obstacle_stock, self.prev_obstacle_stock);
            // eprintln!("height={}", self.board.max_height());
            // eprintln!("{:?}", self.board);

            // self.player.put(&self.packs[self.cur_turn], &act);
            println!("{}", act);
            // assert!(self.cur_turn < 3);
        }
        // while let Some(act) = self.think() {
        //     println!("{}", act);
        // }
    }

    fn think(&mut self) -> action::Action {
        let mut plan_context = PlanContext {

        };

        // foreach actions
        // launch lambda async, and, choose best result
        for i in 0.. {
            let plan_context = PlanContext {

            };

        }
    }
}
