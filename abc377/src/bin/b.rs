use ndarray::prelude::*;
use proconio::{input, marker::Chars};

const N: usize = 8;

fn main() {
    input! {
        ss: [Chars; N],
    }

    let mut mat = Array2::from_elem((N, N), true);
    for row in 0..N {
        if (0..N).any(|col| ss[row][col] == '#') {
            mat.slice_mut(s![row, ..])
                .iter_mut()
                .for_each(|f| *f = false);
        }
    }
    for col in 0..N {
        if (0..N).any(|row| ss[row][col] == '#') {
            mat.slice_mut(s![.., col])
                .iter_mut()
                .for_each(|f| *f = false);
        }
    }

    let ans = mat.iter().filter(|&&f| f).count();
    println!("{}", ans);
}
