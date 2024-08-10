use proconio::input;

fn main() {
    input! {
        (n, t, a): (usize, usize, usize),
    }

    let rem = n - (t + a);
    let ans = t > a + rem || a > t + rem;
    println!("{}", if ans { "Yes" } else { "No" });
}
