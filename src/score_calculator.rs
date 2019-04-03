

use super::action;

thread_local! {
    static SCORE_CALCULATOR: ScoreCalculator = ScoreCalculator::new();
}

const MAX: usize = 200;

pub struct ScoreCalculator {
    bomb_obstacle: Vec<i32>,    // floor(floor(25*2^(b/12))/2)
    chian_obstacle: Vec<i32>,   // floor(sum(i=1,c,floor(1.3^i))/2)
    skill_guage: Vec<i32>,      // sum(i=1,c,floor(1.3^i))
}

impl ScoreCalculator {
    fn new() -> Self {
        let mut pow13 = vec![1_f64; MAX + 1];
        let mut pow12 = vec![1_f64; MAX + 1];
        let mut bomb_obstacle = vec![0; MAX + 1];
        let mut chian_obstacle = vec![0; MAX + 1];
        let mut skill_guage = vec![0; MAX + 1];
        let inv_12 = 2_f64.powf(1.0/12.0);
        let mut sum = 0.0;
        for i in 0..MAX {
            pow13[i+1] = pow13[i] * 1.3;
            pow12[i+1] = pow12[i] * inv_12;
            sum += pow13[i+1].floor();
            let bomb_score = (25.0 * pow12[i+1]).floor();
            bomb_obstacle[i+1] = (bomb_score / 2.0).floor() as i32;
            chian_obstacle[i+1] = (sum / 2.0).floor() as i32;
            if i+1 >= 3 {
                skill_guage[i+1] = 12 + 2 * (i+1) as i32;
            }
        }
        eprintln!("ScoreCalculator: {} {}", bomb_obstacle[5], chian_obstacle[5]);
        Self {
            bomb_obstacle,
            chian_obstacle,
            skill_guage,
        }
    }

    pub fn calc_chain_result(chains: u8) -> action::ActionResult {
        SCORE_CALCULATOR.with(|s| {
            let obstacle = s.chian_obstacle[chains as usize];
            let skill_guage = Self::decrease_skill_guage(chains);
            action::ActionResult::new(chains, obstacle, skill_guage)
        })
    }

    pub fn calc_bomb_result(bomb: u8, chains: u8) -> action::ActionResult {
        SCORE_CALCULATOR.with(|s| {
            let obstacle = s.bomb_obstacle[bomb as usize] + s.chian_obstacle[chains as usize];
            let skill_guage = Self::decrease_skill_guage(chains);
            action::ActionResult::new(chains, obstacle, skill_guage)
        })
    }

    fn decrease_skill_guage(chain: u8) -> i32 {
        if chain < 3 {
            0
        } else {
            12 + 2 * chain as i32
        }
    }
}


#[test]
fn score_calculator_test() {
    SCORE_CALCULATOR.with(|s| {
        assert_eq!(s.bomb_obstacle[5], 16);
        assert_eq!(s.chian_obstacle[5], 4);
    });
}
