use proconio::input;

const MAX: usize = 10_usize.pow(9);

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "infinite".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let sum = aa.iter().sum::<usize>();
    if sum <= m {
        return None;
    }

    let mut ok = 0_usize;
    let mut ng = MAX;

    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;

        let sum = aa.iter().map(|&a| mid.min(a)).sum::<usize>();
        if sum <= m {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    Some(ok)
}
