use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut ss: [Chars; n],
    }

    println!("{}", rec(&mut ss, 0));
}

fn rec(ss: &mut [Vec<char>], pos: usize) -> usize {
    if ss.len() <= 1 {
        return 0;
    }

    ss.sort_unstable_by_key(|s| (s[pos], s.len()));

    let mut score = 0;
    let mut left = 0;
    while left < ss.len() {
        let right = left + ss[left..].partition_point(|s| s[pos] == ss[left][pos]);
        let len = right - left;
        score += len * (len - 1) / 2;
        let next_start = left + ss[left..right].partition_point(|s| s.len() <= pos + 1);
        score += rec(&mut ss[next_start..right], pos + 1);
        left = right;
    }

    score
}
