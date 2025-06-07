use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, l): (usize, usize),
        dd: [usize; n - 1],
    }

    if l % 3 != 0 {
        return 0;
    }

    let mut counts = vec![0_usize; l];
    counts[0] = 1;
    let mut pos = 0_usize;
    for &d in &dd {
        pos = (pos + d) % l;
        counts[pos] += 1;
    }

    let l3 = l / 3;
    (0..l)
        .map(|pos| counts[pos] * counts[(pos + l3) % l] * counts[(pos + 2 * l3) % l])
        .sum::<usize>()
        / 3
}
