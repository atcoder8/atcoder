use itertools::{enumerate, Itertools};
use num::Integer;
use proconio::input;

// 全体集合Uを以下で定義する．
// U = {1, 2, ... , N}
// また、Uの部分集合を定義域とする関数P(I),S(I)を以下で定義する．
// P(I) = {y ∈ {1, 2, ... ,Y} | ∀_{i∈U} i∈I ⇒ A_i|y} (I ⊂ U)
// S(I) = {y ∈ {1, 2, ... ,Y} | ∀_{i∈U} i∈I ⇔ A_i∣y } (I ⊂ U)
// このとき、以下の関係式が成り立つ。
// P(I) = ∪_{J ⊂ U-I} S(I+J) (I ⊂ U)

fn main() {
    input! {
        (n, m, y): (usize, u32, usize),
        aa: [usize; n],
    }

    let mut lcm_by_subset = vec![1_usize; 1 << n];
    for bits in 1_usize..1 << n {
        let lsb = bits & bits.wrapping_neg();
        let from_lcm = lcm_by_subset[bits ^ lsb];
        let a = aa[lsb.trailing_zeros() as usize];
        let gcd = from_lcm.gcd(&a);
        lcm_by_subset[bits] = from_lcm.checked_mul(a / gcd).unwrap_or(y + 1);
    }

    let mut dp = lcm_by_subset.iter().map(|&lcm| y / lcm).collect_vec();
    for i in 0..n {
        for bits in 0..1 << n {
            if bits >> i & 1 == 0 {
                dp[bits] -= dp[bits | 1 << i];
            }
        }
    }

    let ans = enumerate(&dp)
        .map(|(bits, &cnt)| cnt * (bits.count_ones() == m) as usize)
        .sum::<usize>();
    println!("{ans}");
}
