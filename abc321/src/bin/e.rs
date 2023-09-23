use proconio::input;

fn main() {
    input! {
        t: usize,
        nxk: [(usize, usize, usize); t],
    }

    for &(n, x, k) in &nxk {
        println!("{}", solve(n, x, k));
    }
}

fn solve(n: usize, x: usize, k: usize) -> usize {
    let calc_descendants = |root: usize, dist: usize| {
        if dist <= 60 && root <= n >> dist {
            (((root + 1) << dist) - 1).min(n) - (root << dist) + 1
        } else {
            0
        }
    };

    let mut ans = calc_descendants(x, k);
    let mut cur = x;
    for back in 1..=k.saturating_sub(1) {
        if cur == 1 {
            break;
        }

        ans += calc_descendants(cur ^ 1, k - (back + 1));
        cur /= 2;
    }
    if k <= 60 && k != 0 && x >> k != 0 {
        ans += 1;
    }

    ans
}
