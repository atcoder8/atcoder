use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", n.to_string().repeat(n));
}
