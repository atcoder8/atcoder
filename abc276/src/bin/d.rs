use num::Integer;
use proconio::input;

fn main() {
    if let Some(ans) = solve() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    let g = aa.iter().fold(0_usize, |acc, x| acc.gcd(x));
    aa.iter_mut().for_each(|a| *a = *a / g);

    let mut cnt = 0;

    for &a in &aa {
        let mut t = a;

        while t % 2 == 0 {
            t /= 2;
            cnt += 1;
        }

        while t % 3 == 0 {
            t /= 3;
            cnt += 1;
        }

        if t != 1 {
            return None;
        }
    }

    Some(cnt)
}
