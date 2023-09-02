use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        (n, d, p): (usize, usize, usize),
        mut ff: [usize; n],
    }

    ff.sort_unstable_by_key(|&f| Reverse(f));

    let mut ans = 0;
    for chunk in ff.chunks(d) {
        let sum: usize = chunk.iter().sum();
        if p < sum {
            ans += p;
        } else {
            ans += sum;
        }
    }
    println!("{}", ans);
}
