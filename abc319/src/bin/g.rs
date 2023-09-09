// unfinished

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    if uv
        .iter()
        .all(|&edge| edge != (0, n - 1) && edge != (n - 1, 0))
    {
        println!("{}", 1);
        std::process::exit(0);
    }
}
