use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut ans = 0;
    let mut counts = vec![0; n];
    let mut contributions = vec![0; n];
    let mut prev: Vec<Option<usize>> = vec![None; n];
    for (i, &a) in aa.iter().enumerate() {
        if let Some(prev) = prev[a] {
            contributions[a] += counts[a] * (i - prev - 1);
        }
        ans += contributions[a];
        counts[a] += 1;
        prev[a] = Some(i);
    }

    println!("{}", ans);
}
