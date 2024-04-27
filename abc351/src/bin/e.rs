use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut coords_each_parity = vec![vec![]; 2];
    for &(x, y) in &xy {
        coords_each_parity[((x + y) % 2 != 0) as usize].push((x, y));
    }

    let mut ans = 0;
    for coords in &coords_each_parity {
        for f in [|(x, y): (i64, i64)| x - y, |(x, y): (i64, i64)| x + y] {
            let ss = coords.iter().cloned().map(f).sorted_unstable();
            let mut sum = 0;
            for (i, s) in enumerate(ss) {
                ans += s * i as i64 - sum;
                sum += s;
            }
        }
    }
    ans /= 2;

    println!("{}", ans);
}
