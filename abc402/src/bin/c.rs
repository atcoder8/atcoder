use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aaa: [[Usize1]; m],
        bb: [Usize1; n],
    }

    let mut labels = vec![0; n];
    for (i, &b) in enumerate(&bb) {
        labels[b] = i;
    }

    let mut counts = vec![0_usize; n];
    for aa in &aaa {
        let overcoming_day = aa.iter().map(|&a| labels[a]).max().unwrap();
        counts[overcoming_day] += 1;
    }

    let ans = counts
        .iter()
        .scan(0_usize, |acc, cnt| {
            *acc += cnt;
            Some(*acc)
        })
        .join("\n");
    println!("{}", ans);
}
