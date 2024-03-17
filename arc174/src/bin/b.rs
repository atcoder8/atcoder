use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    println!("{}", (0..t).map(|_| solve()).join("\n"));
}

fn solve() -> usize {
    input! {
        aa: [usize; 5],
        pp: [usize; 5],
    }

    // 追加する候補は星4と星5のみ
    let sum_a = aa.iter().sum::<usize>();
    let weighted_sum_a = enumerate(&aa).map(|(i, &a)| (i + 1) * a).sum::<usize>();
    let req = (3 * sum_a).saturating_sub(weighted_sum_a);
    let p4 = pp[3];
    let p5 = pp[4];

    let cand1 = req * p4;
    let cand2 = (req + 1) / 2 * p5;
    let cand3 = req / 2 * p5 + req % 2 * p4;

    cand1.min(cand2).min(cand3)
}
