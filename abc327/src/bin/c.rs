use itertools::{iproduct, Itertools};
use proconio::input;

const N: usize = 9;

fn main() {
    input! {
        aaa: [[usize; N]; N],
    }

    println!("{}", if solve(&aaa) { "Yes" } else { "No" });
}

fn solve(aaa: &[Vec<usize>]) -> bool {
    for i in 0..N {
        if !(0..N).map(|j| aaa[i][j]).all_unique() {
            return false;
        }
    }

    for j in 0..N {
        if !(0..N).map(|i| aaa[i][j]).all_unique() {
            return false;
        }
    }

    for (block_row, block_col) in iproduct!(0..3, 0..3) {
        if !iproduct!(0..3, 0..3)
            .map(|(i, j)| aaa[3 * block_row + i][3 * block_col + j])
            .all_unique()
        {
            return false;
        }
    }

    true
}
