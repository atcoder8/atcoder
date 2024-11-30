use std::cmp::Reverse;

use itertools::{enumerate, Itertools};
use proconio::input;
use superslice::Ext;

const MAX: usize = 3 * 10_usize.pow(5);

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        bb: [usize; m],
    }

    let mut acc = vec![MAX; n + 1];
    for (i, &a) in enumerate(&aa) {
        acc[i + 1] = acc[i].min(a);
    }

    let mut ans: Vec<Option<usize>> = vec![None; m];
    for (j, &b) in enumerate(&bb) {
        let pos = acc.lower_bound_by_key(&Reverse(b), |&v| Reverse(v));
        if pos <= n {
            ans[j] = Some(pos);
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|elem| match elem {
                Some(pos) => pos.to_string(),
                None => "-1".to_string(),
            })
            .join("\n")
    );
}
