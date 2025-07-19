use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m],
    }

    let mut dp = vec![];
    for &(a, b) in ab.iter().sorted_unstable_by_key(|&&(a, _)| a) {
        let Some((last_a, last_b)) = dp.last_mut() else {
            dp.push((a, b));
            continue;
        };

        if a - b >= *last_a - *last_b {
            continue;
        }

        if a == *last_a {
            *last_b = b;
        } else {
            dp.push((a, b));
        }
    }

    let mut ans = 0_usize;
    let mut rem = n;
    for &(a, b) in dp.iter().rev() {
        if rem < a {
            continue;
        }

        let num_exchanges = (rem - a) / (a - b) + 1;
        ans += num_exchanges;
        rem -= (a - b) * num_exchanges;
    }
    println!("{}", ans);
}
