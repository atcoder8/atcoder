// unfinished

use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut ans = vec![];
    let mut prev = 0;
    let mut intersection = lr[0];
    for (i, &(l, r)) in enumerate(&lr).skip(1) {
        if l > intersection.1 {
            ans.extend(vec![intersection.1; i - prev]);
            prev = i;
        } else if r < intersection.0 {
            ans.extend(vec![intersection.0; i - prev]);
            prev = i;
        } else {
            intersection.0 = intersection.0.max(l);
            intersection.1 = intersection.1.min(r);
        }
    }
    ans.extend(vec![intersection.0; n - prev]);

    println!("{}", ans.iter().join(" "));
}
