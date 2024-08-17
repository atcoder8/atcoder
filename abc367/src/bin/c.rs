use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        rr: [usize; n],
    }

    let pushed = |seq: &[usize], x: usize| {
        let mut next_seq = seq.to_vec();
        next_seq.push(x);

        next_seq
    };

    let sequences = rr.iter().fold(vec![vec![]], |acc, &r| {
        acc.iter()
            .flat_map(|seq| (1..=r).map(move |x| pushed(seq, x)))
            .collect_vec()
    });

    let ans = sequences
        .iter()
        .filter(|seq| seq.iter().sum::<usize>() % k == 0)
        .map(|seq| seq.iter().join(" "))
        .join("\n");
    println!("{}", ans);
}
