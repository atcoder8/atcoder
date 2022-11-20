use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, l): (usize, usize),
        aa: [usize; n],
    }

    let mut sum = 0;

    for (i, &a) in aa.iter().enumerate() {
        let rem = l - sum;

        if (rem + i) / (i + 1) < a {
            return false;
        }

        sum += a;
    }

    true
}
