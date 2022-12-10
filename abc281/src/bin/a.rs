use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in (0..=n).rev() {
        println!("{}", i);
    }
}
