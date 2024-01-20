use std::io::Write;

use itertools::{enumerate, Itertools};
use proconio::input_interactive;

fn main() {
    input_interactive!(n: usize);

    let m = (0_usize..).find(|&i| (n - 1) >> i == 0).unwrap();
    println!("{}", m);
    std::io::stdout().flush().unwrap();

    for friend in 0..m {
        let juices = (0..n)
            .filter(|&juice| juice >> friend & 1 == 1)
            .collect_vec();
        println!(
            "{} {}",
            juices.len(),
            juices.iter().map(|juice| juice + 1).join(" ")
        );
        std::io::stdout().flush().unwrap();
    }

    input_interactive!(s: String);

    let is_ok =
        |juice: usize| enumerate(s.chars()).all(|(i, c)| (juice >> i & 1 == 1) == (c == '1'));

    let ans = (0..n).find(|&juice| is_ok(juice)).unwrap() + 1;
    println!("{}", ans);
    std::io::stdout().flush().unwrap();
}
