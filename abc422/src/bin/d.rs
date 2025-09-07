use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (u32, usize),
    }

    let aa = (0..n).fold(vec![k], |prev, _| {
        prev.iter().flat_map(|&a| [a / 2, (a + 1) / 2]).collect()
    });
    let u = (k % 2_usize.pow(n) != 0) as usize;
    println!("{u}\n{}", aa.iter().join(" "));
}
