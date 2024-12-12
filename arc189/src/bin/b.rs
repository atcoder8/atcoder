// unfinished

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xx: [usize; n],
    }

    for i in (0..n - 3).rev() {
        let (left, mid1, mid2, right) = xx[i..i + 4].iter().cloned().collect_tuple().unwrap();
        let sym1 = left + right - mid1;
        let sym2 = left + right - mid2;
        if sym2 < mid1 {
            xx[i + 1] = sym2;
            xx[i + 2] = sym1;
        }
    }

    let ans = xx.iter().sum::<usize>();
    println!("{}", ans);
}
