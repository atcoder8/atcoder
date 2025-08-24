use itertools::{enumerate, Itertools};
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, q): (usize, usize),
        mut aa: [usize; n],
        bb: [usize; q],
    }

    aa.sort_unstable();

    let mut acc_a = vec![0_usize; n + 1];
    for (i, &a) in enumerate(&aa) {
        acc_a[i + 1] = acc_a[i] + a;
    }

    let find_min_x = |b: usize| {
        let boundary = aa.lower_bound(&b);

        if boundary == n {
            return None;
        }

        let required = acc_a[boundary] + (b - 1) * (n - boundary) + 1;
        Some(required)
    };

    let output = bb
        .iter()
        .map(|&b| match find_min_x(b) {
            Some(min_x) => min_x.to_string(),
            None => "-1".to_string(),
        })
        .join("\n");
    println!("{output}");
}
