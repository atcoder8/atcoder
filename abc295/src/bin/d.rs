use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let digits: Vec<_> = s.iter().map(|c| c.to_digit(10).unwrap() as usize).collect();

    let mut counts = vec![0_usize; 1024];
    counts[0] = 1;
    let mut bit = 0_usize;
    for &d in &digits {
        bit ^= 1 << d;
        counts[bit] += 1;
    }

    let ans: usize = counts
        .iter()
        .map(|&cnt| cnt * cnt.saturating_sub(1) / 2)
        .sum();
    println!("{}", ans);
}
