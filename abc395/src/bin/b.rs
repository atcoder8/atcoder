use itertools::Itertools;
use ndarray::prelude::*;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut mat = Array2::from_elem((n, n), '\0');
    for i in 1..=n {
        let j = n + 1 - i;
        if i <= j {
            let color = if i % 2 == 1 { '#' } else { '.' };
            mat.slice_mut(s![i - 1..=j - 1, i - 1..=j - 1]).fill(color);
        }
    }

    let ans = mat
        .axis_iter(Axis(0))
        .map(|line| line.iter().join(""))
        .join("\n");
    println!("{}", ans);
}
