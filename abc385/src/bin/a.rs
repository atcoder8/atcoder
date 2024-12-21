use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        values: [usize; 3],
    }

    let sum = values.iter().sum::<usize>();
    let ans = values.iter().all_equal() || values.iter().any(|&value| 2 * value == sum);
    println!("{}", if ans { "Yes" } else { "No" });
}
