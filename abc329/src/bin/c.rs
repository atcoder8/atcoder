use proconio::{input, marker::Bytes};

fn main() {
    input! {
        _n: usize,
        s: Bytes,
    }

    let mut max_counts = vec![0; 26];
    let mut cur = s[0];
    let mut cnt = 1;
    for &c in s[1..].iter() {
        if c != cur {
            max_counts[(cur - b'a') as usize] = max_counts[(cur - b'a') as usize].max(cnt);

            cur = c;
            cnt = 1;
        } else {
            cnt += 1;
        }
    }
    max_counts[(cur - b'a') as usize] = max_counts[(cur - b'a') as usize].max(cnt);

    let ans: usize = max_counts.iter().sum();
    println!("{}", ans);
}
