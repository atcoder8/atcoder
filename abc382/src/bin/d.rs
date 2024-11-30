use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let mut collected = vec![];

    let mut stack = (1..=m).map(|first| vec![first]).collect_vec();
    while let Some(seq) = stack.pop() {
        if seq.len() == n {
            collected.push(seq);
            continue;
        }

        let last = *seq.last().unwrap();
        if last + 10 * (n - seq.len()) > m {
            continue;
        }

        for next in last + 10..=m {
            let mut next_seq = seq.clone();
            next_seq.push(next);
            stack.push(next_seq);
        }
    }

    collected.sort_unstable();

    println!(
        "{}\n{}",
        collected.len(),
        collected.iter().map(|seq| seq.iter().join(" ")).join("\n")
    );
}
