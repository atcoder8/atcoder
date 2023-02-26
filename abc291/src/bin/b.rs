use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xx: [usize; 5 * n],
    }

    xx.sort_unstable();

    let sum: usize = xx[n..(4 * n)].iter().sum();
    let avg = sum as f64 / (3 * n) as f64;
    println!("{}", avg);
}
