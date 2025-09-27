use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let ans = (1..=n)
        .map(|i| (-1_i64).pow(i as u32) * i.pow(3))
        .sum::<i64>();
    println!("{ans}");
}
