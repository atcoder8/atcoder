use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut fs: [(Usize1, usize); n],
    }

    let max_pos = fs.iter().position_max_by_key(|x| x.1).unwrap();
    let (max_f, max_s) = fs[max_pos];
    fs.remove(max_pos);

    let ans = fs
        .iter()
        .map(|&(f, s)| if f != max_f { max_s + s } else { max_s + s / 2 })
        .max()
        .unwrap();
    println!("{}", ans);
}
