use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let max_a = *aa.iter().max().unwrap();

    let mut counts = vec![0_usize; max_a + 1];
    for &a in &aa {
        counts[a] += 1;
    }

    for i in 1..=max_a {
        counts[i] = (i..=max_a).step_by(i).map(|j| counts[j]).sum();
    }

    let mut max_gcd = vec![0_usize; max_a + 1];
    for i in 1..=max_a {
        if counts[i] < k {
            continue;
        }

        for j in (i..=max_a).step_by(i) {
            max_gcd[j] = max_gcd[j].max(i);
        }
    }

    println!("{}", aa.iter().map(|&a| max_gcd[a]).join("\n"));
}
