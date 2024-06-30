use itertools::izip;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
        ww: [usize; n],
    }

    let mut ww_each_box = vec![vec![]; n];
    for (&a, &w) in izip!(&aa, &ww) {
        ww_each_box[a].push(w);
    }

    let ans = ww_each_box
        .iter()
        .filter(|ww| !ww.is_empty())
        .map(|ww| ww.iter().sum::<usize>() - ww.iter().max().unwrap())
        .sum::<usize>();
    println!("{}", ans);
}
