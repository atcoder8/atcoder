use proconio::input;

fn main() {
    input! {
        a: usize,
    }

    println!(
        "{}",
        if 400 % a == 0 {
            (400 / a).to_string()
        } else {
            "-1".to_string()
        }
    );
}
