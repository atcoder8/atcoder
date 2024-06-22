use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        hh: [usize; n],
    }

    let mut ans = vec![0; n];
    let mut stack: Vec<(usize, usize)> = vec![];
    let mut cur = 0;
    for (i, &h) in enumerate(&hh) {
        let mut dist = 1;
        while stack.last().is_some_and(|&(v, _)| v < h) {
            let (v, d) = stack.pop().unwrap();
            cur -= v * d;
            dist += d;
        }

        cur += dist * h;
        ans[i] = cur + 1;

        stack.push((h, dist));
    }

    println!("{}", ans.iter().join(" "));
}
