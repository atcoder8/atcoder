use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (_n, q): (usize, usize),
        s: Chars,
        lr: [(Usize1, usize); q],
    }

    let mut counts = vec![0];
    for win in s.windows(2) {
        counts.push(counts.last().unwrap() + (win[0] == win[1]) as usize);
    }

    for &(l, r) in &lr {
        let ans = counts[r - 1] - counts[l];
        println!("{}", ans);
    }
}
