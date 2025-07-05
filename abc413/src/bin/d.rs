use itertools::Itertools;
use proconio::input;

// 全て符号が同じ、または交互
// 絶対値が大きくなる数列のみを考える
// 最初の項が正か負かで場合分け

fn main() {
    input! {
        t: usize,
    }

    println!(
        "{}",
        (0..t)
            .map(|_| if solve() { "Yes" } else { "No" })
            .join("\n")
    );
}

fn solve() -> bool {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut positive = vec![];
    let mut negative = vec![];
    for &a in &aa {
        if a > 0 {
            positive.push(a as usize);
        } else {
            negative.push(-a as usize);
        }
    }
    positive.sort_unstable();
    negative.sort_unstable();

    // 全ての符号が正の場合
    if negative.len() == 0 {
        return positive
            .iter()
            .tuple_windows()
            .all(|(a1, a2, a3)| a1 * a3 == a2.pow(2));
    }

    // 全ての符号が負の場合
    if positive.len() == 0 {
        return negative
            .iter()
            .tuple_windows()
            .all(|(a1, a2, a3)| a1 * a3 == a2.pow(2));
    }

    if positive.len().abs_diff(negative.len()) > 1 {
        return false;
    }

    if positive.len() < negative.len() {
        std::mem::swap(&mut positive, &mut negative);
    }

    if positive
        .iter()
        .interleave(&negative)
        .tuple_windows()
        .all(|(a1, a2, a3)| a1 * a3 == a2.pow(2))
    {
        return true;
    }

    if positive.len() == negative.len() {
        return negative
            .iter()
            .interleave(&positive)
            .tuple_windows()
            .all(|(a1, a2, a3)| a1 * a3 == a2.pow(2));
    }

    false
}
