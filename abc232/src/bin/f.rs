use proconio::input;

const INF: i64 = 2_000_000_000_000_000_000;

fn main() {
    input! {
        (n, x, y): (usize, i64, i64),
        aa: [i64; n],
        bb: [i64; n],
    }

    let mut dp = vec![INF; 1 << n];
    dp[0] = 0;
    for bit in 0..(1_usize << n) {
        let cnt = bit.count_ones() as usize;
        for p in 0..n {
            if (bit >> p) & 1 == 1 {
                continue;
            }
            let next_bit = bit + (1 << p);
            let next_cost = dp[bit]
                + (aa[p] - bb[cnt]).abs() * x
                + (bit & ((1_usize << n) - (1_usize << p))).count_ones() as i64 * y;
            dp[next_bit] = std::cmp::min(dp[next_bit], next_cost);
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}
