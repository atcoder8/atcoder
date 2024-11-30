use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (_n, d): (usize, usize),
        s: String,
    }

    let mut rem = d;
    let mut ans = s
        .chars()
        .rev()
        .map(|c| {
            if c == '.' {
                '.'
            } else if rem >= 1 {
                rem -= 1;
                '.'
            } else {
                '@'
            }
        })
        .collect_vec();
    ans.reverse();
    println!("{}", ans.iter().collect::<String>());
}
