use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let sum_a = aa.iter().sum::<usize>();
    println!("{}", if sum_a <= m { "Yes" } else { "No" });
}
