use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!(
        "{}{}",
        std::char::from_digit((n / 16) as u32, 16)
            .unwrap()
            .to_uppercase(),
        std::char::from_digit((n % 16) as u32, 16)
            .unwrap()
            .to_uppercase()
    );
}
