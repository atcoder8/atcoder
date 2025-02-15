use itertools::{enumerate, Itertools};
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [usize; n],
        rx: [(usize, usize); q],
    }

    let queries = enumerate(&rx)
        .map(|(i, &(r, x))| Query::new(i, r, x))
        .sorted_unstable_by_key(|query| query.length)
        .collect_vec();

    let mut ans = vec![0_usize; q];
    let mut progress = 0_usize;
    let mut dp = vec![];
    for (i, &a) in enumerate(&aa) {
        let idx = dp.lower_bound(&a);
        if idx < dp.len() {
            dp[idx] = a;
        } else {
            dp.push(a);
        }

        let length = i + 1;
        while progress < q && queries[progress].length <= length {
            let query = queries[progress];
            ans[query.id] = dp.upper_bound(&query.max);
            progress += 1;
        }
    }

    println!("{}", ans.iter().join("\n"));
}

#[derive(Debug, Clone, Copy)]
struct Query {
    id: usize,
    length: usize,
    max: usize,
}

impl Query {
    fn new(id: usize, length: usize, max: usize) -> Self {
        Self { id, length, max }
    }
}
