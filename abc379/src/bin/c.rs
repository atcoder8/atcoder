use itertools::{izip, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
        xx: [Usize1; m],
        aa: [usize; m],
    }

    let mut xa = izip!(xx, aa)
        .sorted_unstable_by_key(|&(x, _)| x)
        .collect_vec();
    xa.push((n, 0));

    let mut min_cost = 0_usize;
    let mut prev_pos = 0_usize;
    let mut rem = 0_usize;
    for &(pos, num) in &xa {
        let diff = pos - prev_pos;

        if rem < diff {
            return None;
        }

        min_cost += diff * rem - diff * (diff + 1) / 2;
        rem = rem - diff + num;
        prev_pos = pos;
    }

    if rem == 0 {
        Some(min_cost)
    } else {
        None
    }
}
