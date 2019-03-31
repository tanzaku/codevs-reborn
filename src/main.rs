

mod action;
mod board;
mod rensa_plan;
mod rensa_ai;
mod rand;


fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();
    let mut ai = rensa_ai::RensaAi::new(lock);
    ai.exec();
}
