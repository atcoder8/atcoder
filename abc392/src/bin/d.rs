use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aaa: [[u32]; n],
    }

    let dices = aaa
        .iter()
        .map(|rolls| Dice::from_rolls(rolls))
        .collect_vec();
    let ans = dices
        .iter()
        .tuple_combinations()
        .map(|(dice1, dice2)| dice1.match_probability(dice2))
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    println!("{}", ans);
}

#[derive(Debug, Clone)]
struct Dice {
    probably_by_roll: BTreeMap<u32, f64>,
}

impl Dice {
    fn from_rolls(rolls: &[u32]) -> Self {
        let p = 1.0 / rolls.len() as f64;
        let mut probably_by_roll = BTreeMap::<u32, f64>::new();
        for &roll in rolls {
            *probably_by_roll.entry(roll).or_default() += p;
        }

        Self { probably_by_roll }
    }

    fn num_roll_types(&self) -> usize {
        self.probably_by_roll.len()
    }

    fn match_probability(&self, other: &Self) -> f64 {
        if self.num_roll_types() > other.num_roll_types() {
            return other.match_probability(self);
        }

        self.probably_by_roll
            .iter()
            .map(|(&roll, &prob)| prob * other.probably_by_roll.get(&roll).unwrap_or(&0.0))
            .sum::<f64>()
    }
}
