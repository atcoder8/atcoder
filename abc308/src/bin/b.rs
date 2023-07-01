use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        cc: [String; n],
        dd: [String; m],
        pp: [usize; m + 1],
    }

    let mut ans = 0;
    for c in &cc {
        let pos = dd.iter().find_position(|&d| c == d);
        if let Some(pos) = pos {
            ans += pp[pos.0 + 1];
        } else {
            ans += pp[0];
        }
    }
    println!("{}", ans);
}
