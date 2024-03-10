use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let ans = (0..t).map(|_| solve()).join("\n");
    println!("{}", ans);
}

fn solve() -> usize {
    input! {
        k: usize,
    }

    // key以下のNeq Numberがk個以上存在するか
    let is_ok = |key: usize| {
        let digits = key
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect_vec();

        let mut less = vec![0; 10];
        less[1..digits[0]].fill(1);
        let mut equal = Some(digits[0]);
        for &d in &digits[1..] {
            let sum_less = less.iter().sum::<usize>();
            let mut next_less = (0..10)
                .map(|i| (i != 0) as usize + sum_less - less[i])
                .collect_vec();
            if let Some(equal) = equal {
                (0..d)
                    .filter(|&i| i != equal)
                    .for_each(|i| next_less[i] += 1);
            }
            less = next_less;

            equal = if equal.is_some_and(|equal| d != equal) {
                Some(d)
            } else {
                None
            };
        }

        less.iter().sum::<usize>() + equal.is_some() as usize >= k
    };

    let mut ok = 10_usize.pow(18);
    let mut ng = 0_usize;
    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    ok
}
