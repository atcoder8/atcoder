use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut aa: [Usize1; n],
    }

    let mut positions = vec![0; n];
    for (i, &a) in enumerate(&aa) {
        positions[a] = i;
    }

    let mut ans = vec![];
    for i in 0..n {
        let cur = aa[i];
        let pos = positions[i];

        if pos == i {
            continue;
        }

        aa.swap(i, pos);
        positions.swap(cur, i);

        ans.push((i, pos));
    }

    println!(
        "{}\n{}",
        ans.len(),
        ans.iter()
            .map(|(l, r)| format!("{} {}", l + 1, r + 1))
            .join("\n")
    );
}
