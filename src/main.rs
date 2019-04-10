
#![allow(dead_code)]
// #![allow(unused_variables)]

extern crate codevs_reborn_lib;

// mod action;
// mod board;
// mod skill_plan;
// mod rensa_plan;
// mod rensa_ai;
// mod skill_ai;
// mod rand;
// mod score_calculator;
// mod player;

use codevs_reborn_lib::rensa_ai;

fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();
    let mut ai = rensa_ai::RensaAi::new(lock);
    // let mut ai = skill_ai::RensaAi::new(lock);
    ai.exec();
}
