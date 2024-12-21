use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize,
        hh: [usize; n],
    }

    let count_score = |start: usize, interval: usize| {
        let expected_height = hh[start];
        (start..n)
            .step_by(interval)
            .take_while(|&i| hh[i] == expected_height)
            .count()
    };

    let ans = iproduct!(0..n, 1..=n)
        .map(|(start, interval)| count_score(start, interval))
        .max()
        .unwrap();
    println!("{}", ans);
}
