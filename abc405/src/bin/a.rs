use proconio::input;

fn main() {
    input! {
        (r, x): (usize, usize),
    }

    let div1 = 1600..3000;
    let div2 = 1200..2400;
    let ans = [div1, div2][x - 1].contains(&r);
    println!("{}", if ans { "Yes" } else { "No" });
}
