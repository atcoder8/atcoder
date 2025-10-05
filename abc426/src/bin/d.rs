use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> usize {
    input! {
        n: usize,
        s: Chars,
    }

    let calc_acc_costs = |s: &[char], dest: char| {
        let mut costs = vec![0_usize; n + 1];
        let mut last_error_pos = 0_usize;
        for i in 1..=n {
            let ch = s[i - 1];
            if ch == dest {
                costs[i] = costs[i - 1];
            } else {
                costs[i] = costs[i - 1] + 2 * (i - 1 - last_error_pos) + 1;
                last_error_pos = i;
            }
        }

        costs
    };

    let find_min_cost = |dest: char| {
        let mut s = s.clone();

        let forward_costs = calc_acc_costs(&s, dest);
        s.reverse();
        let backward_costs = calc_acc_costs(&s, dest);

        (0..=n)
            .map(|i| forward_costs[i] + backward_costs[n - i])
            .min()
            .unwrap()
    };

    ['0', '1']
        .iter()
        .map(|&dest| find_min_cost(dest))
        .min()
        .unwrap()
}
