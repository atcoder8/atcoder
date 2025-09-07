use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let (mut world, mut stage) = s
        .split("-")
        .map(|v| v.parse::<u8>().unwrap())
        .collect_tuple()
        .unwrap();
    stage += 1;
    if stage > 8 {
        (world, stage) = (world + 1, 1);
    }
    println!("{world}-{stage}");
}
