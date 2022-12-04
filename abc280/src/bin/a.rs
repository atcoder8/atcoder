use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, _w): (usize, usize),
        ss: [Chars; h],
    }

    let ans: usize = ss
        .iter()
        .map(|s| s.iter().filter(|&&c| c == '#').count())
        .sum();
    println!("{}", ans);
}
