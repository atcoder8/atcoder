use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        s: Chars,
    }

    let mut dp = vec![vec![0_usize; 8]; 3];
    for i in 0..n {
        let mut next_dp = dp.clone();

        let step = char_to_usize(s[i]);

        if step == 0 {
            next_dp[0][1 << aa[i]] += 1;
        } else {
            for bit in 0..8 {
                next_dp[step][bit | (1 << aa[i])] += dp[step - 1][bit];
            }
        }

        dp = next_dp;
    }

    let mut ans = 0;
    for bit in 0..8 {
        let mex = (0..=3).find(|&x| (bit >> x) & 1 == 0).unwrap();
        ans += dp[2][bit] * mex;
    }
    println!("{}", ans);
}

fn char_to_usize(c: char) -> usize {
    match c {
        'M' => 0,
        'E' => 1,
        'X' => 2,
        _ => panic!(),
    }
}
