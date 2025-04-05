use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = (1..=n.sqrt())
        .step_by(2)
        .map(|b: usize| (0_usize..).find(|&i| (n / b.pow(2)) >> i == 1).unwrap())
        .sum::<usize>();
    println!("{}", ans);
}
