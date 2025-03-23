use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let num_equals = 2 - n % 2;
    let num_minuses = (n - num_equals) / 2;
    println!("{0}{1}{0}", "-".repeat(num_minuses), "=".repeat(num_equals));
}
