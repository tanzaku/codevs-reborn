


use std::ops::{Index,IndexMut};
use std::collections::HashSet;

use super::action;
use super::score_calculator;


const DEAD_LINE_Y: i32 = 16;
const W: i32 = 11;
const H: i32 = DEAD_LINE_Y + 3;
const DIRS: [i32; 4] = [1, 1 + W, W, W-1];
const DIRS9: [i32; 9] = [W+1, W, W-1, 1, 0, -1, -W+1, -W, -W-1];
const VANISH: u8 = 10;
const OBSTACLE: u8 = VANISH + 1;

// struct Cookie<T> {
//     data: Vec<T>,
//     id: Vec<u32>,
//     gid: u32,
// }

// impl<T> Cookie<T> where T: Clone + Default {
//     pub fn new(capacity: usize) -> Self {
//         Self {
//             data: vec![Default::default(); capacity],
//             id: vec![0; capacity],
//             gid: 0,
//         }
//     }

//     pub fn get(&self, i: usize) -> T {
//         if self.id[i] == self.gid {
//             self.data[i].clone()
//         } else {
//             Default::default()
//         }
//     }

//     pub fn set(&mut self, i: usize, v: T) {
//         self.id[i] = self.gid;
//         self.data[i] = v;
//     }

//     pub fn clear(&mut self) {
//         self.gid += 1;
//     }
// }

#[derive(Clone)]
pub struct Board {
    // column: [u64; W],
    board: [u8; (W * H) as usize],
    height: [u8; W as usize],
}

fn rotate(pattern: &[[u8; 2]; 2], rot: usize) -> [[u8; 2]; 2] {
    let mut rot = rot;
    let mut pattern = pattern.clone();
    while rot > 0 {
        pattern = [[pattern[1][0], pattern[0][0]],
                    [pattern[1][1], pattern[0][1]]];
        rot -= 1;
    }
    pattern
}

impl Board {
    pub fn new() -> Self {
        let mut b = [0; (W * H) as usize];
        for y in 0..H {
            b[(y*W+W-1) as usize] = VANISH;
        }
        Self {
            board: b,
            height: [0; W as usize],
        }
    }

    pub fn from_board(board: [u8; ((W-1) * (H-3)) as usize]) -> Self {
        let mut b = [0; (W * H) as usize];
        let mut h = [0; W as usize];
        for y in 0..H {
            b[(y*W+W-1) as usize] = VANISH;
        }
        for y in 0..H-3 {
            for x in 0..W-1 {
                b[(y*W+x) as usize] = board[((H-3-1-y)*(W-1)+x) as usize];
                if b[(y*W+x) as usize] != 0 {
                    h[x as usize] = (y + 1) as u8;
                }
            }
        }
        Self {
            board: b,
            height: h,
        }
    }

    fn top_empty_pos(&self, x: usize) -> usize {
        self.height[x] as usize * W as usize + x
    }

    pub fn put(&mut self, pattern: &[[u8; 2]; 2], pos: usize, rot: usize) -> action::ActionResult {
        assert!(pos + pattern.len() <= W as usize);
        let pattern = rotate(pattern, rot);
        let mut dh = [0; 2];
        for dx in 0..pattern[0].len() {
            let x = pos + dx;
            for dy in (0..pattern.len()).rev() {
                if pattern[dy][dx] != 0 {
                    let p = self.top_empty_pos(x);
                    self.board[p] = pattern[dy][dx];
                    self.height[x] += 1;
                    dh[dx] += 1;
                }
            }
        }
        let chains = self.vanish();
        score_calculator::ScoreCalculator::calc_chain_result(chains)
    }

    pub fn use_skill(&mut self) -> action::ActionResult {
        let mut set = HashSet::new();
        for x in 0..W-1 {
            for y in 0..self.height[x as usize] as i32 {
                let p = y * W + x;
                if self[p] != 5 {
                    continue;
                }
                DIRS9.into_iter().filter(|d| self[p+*d] < VANISH).for_each(|d| { set.insert(p + d); });
            }
        }
        if set.is_empty() {
            return action::ActionResult::new(0, 0, 0);
        }
        set.iter().for_each(|p| { self[*p] = 0; });
        self.fall_down();
        let bombed_block = set.len() as u8;
        let chains = self.vanish();
        score_calculator::ScoreCalculator::calc_bomb_result(bombed_block, chains)
    }

    fn vanish(&mut self) -> u8 {
        let mut rensa = 0;
        loop {
            let mut set = HashSet::new();
            for x in 0..W-1 {
                for y in 0..self.height[x as usize] as i32 {
                    for d in DIRS.iter() {
                        let l = y * W + x;
                        let r = l + d;
                        if self[l] + self[r] == VANISH {
                            set.insert(l);
                            set.insert(r);
                        }
                    }
                }
            }
            if set.is_empty() {
                break;
            }
            rensa += 1;
            set.into_iter().for_each(|p| { self[p] = 0; });
            self.fall_down();
        }
        rensa
    }

    fn fall_down(&mut self) {
        for x in 0..W-1 {
            let mut h = 0;
            for y in 0..self.height[x as usize] as i32 {
                if y != h {
                    self[(x,h)] = self[(x,y)];
                    self[(x,y)] = 0;
                }
                if self[(x,h)] != 0 {
                    h += 1;
                }
            }
            self.height[x as usize] = h as u8;
        }
    }

    pub fn fall_obstacle(&mut self) {
        if self.is_dead() {
            return;
        }
        for x in 0..W-1 {
            let y = self.height[x as usize] as i32;
            self[(x,y)] = OBSTACLE;
            self.height[x as usize] += 1;
        }
    }

    pub fn is_dead(&self) -> bool {
        *self.height.iter().max().unwrap() as i32 > DEAD_LINE_Y
    }

    pub fn max_height(&self) -> u8 {
        *self.height.iter().max().unwrap()
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
        let mut res = String::new();
        for y in (0..H).rev() {
            for x in 0..W-1 {
                let c = if self[(x,y)] > VANISH { 'X' } else { std::char::from_digit(self[(x,y)] as u32, 10).unwrap() };
                res += &c.to_string();
            }
            res += "\n";
        }
        write!(f, "{}", res)
    }
}

impl Index<i32> for Board {
    type Output = u8;

    fn index(&self, ix: i32) -> &Self::Output {
        // if ix as usize >= self.board.len() {
        //     eprintln!("{:?}", self);
        //     eprintln!("max height: {}", self.height.iter().max().unwrap());
        // }
        if ix < 0 || ix as usize >= self.board.len() {
            return &OBSTACLE;
        }
        &self.board[ix as usize]
    }
}


impl IndexMut<i32> for Board {
    fn index_mut(&mut self, ix: i32) -> &mut u8 {
        &mut self.board[ix as usize]
    }
}

impl Index<(i32,i32)> for Board {
    type Output = u8;

    fn index(&self, ix: (i32,i32)) -> &Self::Output {
        &self.board[(ix.1*W+ix.0) as usize]
    }
}

impl IndexMut<(i32,i32)> for Board {
    fn index_mut(&mut self, ix: (i32,i32)) -> &mut u8 {
        &mut self.board[(ix.1*W+ix.0) as usize]
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.board[..] == other.board[..]
    }
}

impl Eq for Board {}

#[test]
fn board_test() {
    let mut board = Board::new();
    let rensa = board.put(&[[9,5],[0,3]], 8, 3);
    // eprintln!("{:?}", board);
    assert_eq!(board[(8,0)], 9);
    assert_eq!(board[(9,0)], 3);
    assert_eq!(board[(8,1)], 5);
    assert_eq!(board[(9,1)], 0);
    assert_eq!(rensa.obstacle, 0);
    assert_eq!(rensa.skill_guage, 0);
}

#[test]
fn board_test_vanish() {
    let mut board = Board::new();
    let rensa = board.put(&[[9,5],[0,1]], 8, 3);
    // eprintln!("{:?}", board);
    assert_eq!(board[(8,0)], 5);
    assert_eq!(board[(9,0)], 0);
    assert_eq!(board[(8,1)], 0);
    assert_eq!(board[(9,1)], 0);
    assert_eq!(rensa.obstacle, 0);
    assert_eq!(rensa.skill_guage, 0);
}

#[test]
fn board_test_vanish2() {
    let mut board = Board::new();
    let rensa = board.put(&[[2,0],[5,2]], 1, 0);
    assert_eq!(board.max_height(), 2);
    let rensa = board.put(&[[1,0],[5,6]], 1, 0);
    assert_eq!(board.max_height(), 4);
}
