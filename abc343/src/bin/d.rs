use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, t): (usize, usize),
        ab: [(Usize1, usize); t],
    }

    let mut scores = vec![0; n];
    let mut map = BTreeMap::<usize, usize>::from([(0, n)]);
    for &(player, score) in &ab {
        let prev_score = scores[player];
        scores[player] += score;

        let cnt = map.get_mut(&prev_score).unwrap();
        *cnt -= 1;
        if *cnt == 0 {
            map.remove(&prev_score);
        }

        *map.entry(scores[player]).or_default() += 1;

        println!("{}", map.len());
    }
}
