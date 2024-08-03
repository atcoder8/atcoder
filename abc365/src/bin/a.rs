use proconio::input;

fn main() {
    input! {
        y: usize,
    }

    let ans = if y % 400 == 0 {
        366
    } else if y % 100 == 0 {
        365
    } else if y % 4 == 0 {
        366
    } else {
        365
    };
    println!("{}", ans);
}
