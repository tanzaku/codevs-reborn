
extern crate codevs_reborn_lib;


use std::fs::File;
use std::io::BufReader;


use codevs_reborn_lib::best_ai;


fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();
    let mut ai = best_ai::BestAi::new(lock);
    ai.exec();
}


#[test]
#[ignore]
fn rensa_test79() {
    use rayon::prelude::*;
    let filename = format!("in/in_79.txt");
    let file = File::open(filename.clone()).expect("ok");
    let reader = BufReader::new(file);
    let mut ai = best_ai::BestAi::new(reader);
    let chains = ai.rensa_search_test();
    eprintln!("chains: {} {:?}", filename, chains);
}

#[test]
#[ignore]
fn rensa_test76() {
    use rayon::prelude::*;
    let filename = format!("in/in_76.txt");
    let file = File::open(filename.clone()).expect("ok");
    let reader = BufReader::new(file);
    let mut ai = best_ai::BestAi::new(reader);
    let chains = ai.rensa_search_test();
    eprintln!("chains: {} {:?}", filename, chains);
}

#[test]
#[ignore]
fn rensa_test() {
    use rayon::prelude::*;
    let res: Vec<_> = (1..201).into_par_iter().map(|i| {
        let filename = format!("in/in_{}.txt", i);
        let file = File::open(filename.clone()).expect("ok");
        let reader = BufReader::new(file);
        let mut ai = best_ai::BestAi::new(reader);
        let chains = ai.rensa_search_test();
        eprintln!("chains: {} {:?}", filename, chains);
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

#[test]
#[ignore]
fn rensa_test_best() {
    use rayon::prelude::*;
    let res: Vec<_> = (1..201).into_par_iter().map(|i| {
        let filename = format!("in/in_{}.txt", i);
        let file = File::open(filename.clone()).expect("ok");
        let reader = BufReader::new(file);
        let mut ai = best_ai::BestAi::new(reader);
        let best = ai.rensa_search_best_test();
        eprintln!("chains: {} {} {}", filename, best.len(), best.get_chains());
        best
    }).collect();

    let mut cnt = vec![vec![0; 20]; 13];
    res.into_iter().for_each(|r| {
        cnt[r.len() - 1][r.get_chains() as usize] += 1;
    });

    cnt.into_iter().for_each(|c| {
        eprintln!("{}", c.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));
    });
}

#[test]
fn rensa_test_best_merge() {
    use rayon::prelude::*;
    let res: Vec<_> = (1..201).into_par_iter().map(|i| {
        let filename = format!("in/in_{}.txt", i);
        let file = File::open(filename.clone()).expect("ok");
        let reader = BufReader::new(file);
        let mut ai = best_ai::BestAi::new(reader);
        let best = ai.rensa_search_best_test2();
        eprintln!("chains: {} {} {}", filename, best.len(), best.get_chains());
        best
    }).collect();

    let mut cnt = vec![vec![0; 20]; 13];
    res.into_iter().for_each(|r| {
        cnt[r.len() - 1][r.get_chains() as usize] += 1;
    });

    cnt.into_iter().for_each(|c| {
        eprintln!("{}", c.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));
    });
}
