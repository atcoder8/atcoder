use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    let min_votes = solve();
    let ans = min_votes
        .iter()
        .map(|min_vote| match min_vote {
            Some(min_vote) => min_vote.to_string(),
            None => "-1".to_string(),
        })
        .join(" ");
    println!("{}", ans);
}

fn solve() -> Vec<Option<usize>> {
    input! {
        (n, m, k): (usize, usize, usize),
        aa: [usize; n],
    }

    if m == n {
        return vec![Some(0); n];
    }

    let top_votes = aa
        .iter()
        .cloned()
        .sorted_unstable_by_key(|&a| Reverse(a))
        .collect_vec();
    let mut acc_top_votes = vec![0_usize; n + 1];
    for i in 0..n {
        acc_top_votes[i + 1] = acc_top_votes[i] + top_votes[i];
    }

    let sum_a = aa.iter().sum::<usize>();
    let rem_votes = k - sum_a;

    let is_ok = |target_vote: usize, add_vote: usize| {
        let obtained_vote = target_vote + add_vote;
        let other_vote = rem_votes - add_vote;
        let exceeded_num = top_votes.partition_point(|&vote| vote > obtained_vote);
        if exceeded_num >= m {
            return false;
        }
        let offset = (target_vote > top_votes[m]) as usize;
        let sum_opponent_vote =
            acc_top_votes[m + offset] - acc_top_votes[exceeded_num] - offset * target_vote;
        let length = m - exceeded_num;
        (obtained_vote + 1) * length - sum_opponent_vote > other_vote
    };

    let sub_solve = |i: usize| {
        if is_ok(aa[i], 0) {
            return Some(0);
        }

        if !is_ok(aa[i], rem_votes) {
            return None;
        }

        let mut ok = rem_votes;
        let mut ng = 0_usize;

        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) / 2;

            if is_ok(aa[i], mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        Some(ok)
    };

    (0..n).map(sub_solve).collect()
}
