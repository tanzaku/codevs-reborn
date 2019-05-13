
extern crate codevs_reborn_lib;


use std::fs::File;
use std::io::BufReader;

use rayon::prelude::*;

use codevs_reborn_lib::best_ai;

fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();
    let mut ai = best_ai::BestAi::new(lock);
    ai.exec();
}


#[test]
fn rensa_test() {
    let res: Vec<_> = (1..101).into_par_iter().map(|i| {
        let filename = format!("in/in_{}.txt", i);
        let file = File::open(filename).expect("ok");
        let reader = BufReader::new(file);
        let mut ai = best_ai::BestAi::new(reader);
        let chains = ai.rensa_search_test();
        eprintln!("chains: {:?}", chains);
        chains
    }).collect();

    let mut cnt = vec![vec![0; 20]; 13];
    res.into_iter().for_each(|r| {
        r.into_iter().enumerate().for_each(|(i,c)| {
            cnt[i][c as usize] += 1;
        });
    });

    cnt.into_iter().for_each(|c| {
        eprintln!("{}", c.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));
    });
}
