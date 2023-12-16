use proconio::{input, marker::Bytes};

fn main() {
    input! {
        ss: Bytes,
        tt: Bytes,
    }

    let ans = calc_dist(&ss) == calc_dist(&tt);
    println!("{}", if ans { "Yes" } else { "No" });
}

fn calc_dist(ss: &[u8]) -> u8 {
    let diff = ss[0].abs_diff(ss[1]);

    diff.min(5 - diff)
}
