use std::cmp::Reverse;

use itertools::enumerate;
use proconio::{input, marker::Usize1};
use smallvec::SmallVec;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

#[derive(Debug, Clone, Default)]
struct TopScores(SmallVec<[(usize, usize); 3]>);

impl TopScores {
    fn push(&mut self, score: usize, color: usize) {
        let same_color_score = self.0.iter_mut().find(|(_, c)| *c == color);
        match same_color_score {
            Some((s, _)) => *s = (*s).max(score),
            None => self.0.push((score, color)),
        }

        self.0.sort_unstable_by_key(|&(s, _)| Reverse(s));
        self.0.truncate(2);
    }

    fn max_score(&self) -> Option<usize> {
        self.0.get(0).map(|&(score, _)| score)
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, k): (usize, usize),
        cv: [(Usize1, usize); n],
    }

    let mut top_scores_vec = vec![TopScores::default(); k + 1];
    top_scores_vec[0].push(0, n);

    for (ball_idx, &(c, v)) in enumerate(&cv) {
        for remove_num in (0..=ball_idx.min(k)).rev() {
            let top_scores = std::mem::take(&mut top_scores_vec[remove_num]);

            for &(score, color) in &top_scores.0 {
                if color != c {
                    top_scores_vec[remove_num].push(score + v, c);
                }

                if remove_num < k {
                    top_scores_vec[remove_num + 1].push(score, color);
                }
            }
        }
    }

    top_scores_vec[k].max_score()
}
