use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut counts = aa.clone();
    let mut events = vec![0_i64; n + 1];
    let mut acc = 0_i64;
    for i in 0..n {
        acc += events[i];
        counts[i] += acc as usize;
        events[i + 1] += 1;
        events[(i + 1 + counts[i] as usize).min(n)] -= 1;
        counts[i] = counts[i].saturating_sub(n - 1 - i);
    }

    println!("{}", counts.iter().join(" "));
}
