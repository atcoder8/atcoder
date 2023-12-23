use proconio::input;

fn main() {
    input! {
        (b, g): (usize, usize),
    }

    let ans = if b > g { "Bat" } else { "Glove" };
    println!("{}", ans);
}
