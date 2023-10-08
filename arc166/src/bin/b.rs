// unfinished

use std::collections::BTreeSet;

use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        divisors: [usize; 3],
        aa: [usize; n],
    }

    let calc_op_num = |a: usize, divisor: usize| (a + divisor - 1) / divisor * divisor - a;

    let mut op_counts = vec![vec![0; n]; 8];
    for (i, &a) in aa.iter().enumerate() {
        for bit in 1..8 {
            let lcm = (0..3)
                .filter(|&j| (bit >> j) & 1 == 1)
                .fold(1, |acc, j| acc.lcm(&divisors[j]));
            op_counts[bit][i] = calc_op_num(a, lcm);
        }
    }

    let mut op_counts_set = vec![BTreeSet::new(); 8];
    for bit in 0..8 {
        op_counts_set[bit] = op_counts[bit]
            .iter()
            .enumerate()
            .map(|(i, &op_cnt)| (op_cnt, i))
            .collect();
    }

    let ans = dfs(&op_counts, &mut op_counts_set, 0).unwrap();
    println!("{}", ans);
}

fn dfs(
    op_counts: &Vec<Vec<usize>>,
    op_counts_set: &mut Vec<BTreeSet<(usize, usize)>>,
    satisfied: usize,
) -> Option<usize> {
    if satisfied == 7 {
        return Some(0);
    }

    for bit in 1..8 {
        if bit & satisfied == 0 {
            if let Some(&(op_cnt_1, idx)) = op_counts_set[bit].first() {
                op_counts_set
                    .iter_mut()
                    .enumerate()
                    .for_each(|(bit, op_cnt)| {
                        op_cnt.remove(&(op_counts[bit][idx], idx));
                    });

                if let Some(op_cnt_2) = dfs(op_counts, op_counts_set, satisfied | bit) {
                    return Some(op_cnt_1 + op_cnt_2);
                }

                op_counts_set
                    .iter_mut()
                    .enumerate()
                    .for_each(|(bit, op_cnt)| {
                        op_cnt.insert((op_counts[bit][idx], idx));
                    });
            }
        }
    }

    None
}
