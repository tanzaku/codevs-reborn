
extern crate codevs_reborn_lib;

use codevs_reborn_lib::best_ai;

fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();
    let mut ai = best_ai::BestAi::new(lock);
    ai.exec();
}
