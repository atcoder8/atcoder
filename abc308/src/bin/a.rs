use proconio::input;

fn main() {
    input! {
        ss: [usize; 8],
    }

    let ans = ss.windows(2).all(|window| window[0] <= window[1])
        && ss.iter().all(|&s| s >= 100 && s <= 675 && s % 25 == 0);
    println!("{}", if ans { "Yes" } else { "No" });
}
