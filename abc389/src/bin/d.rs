use proconio::input;

const MAX: i64 = 10_i64.pow(6);

fn main() {
    input! {
        r: i64,
    }

    let is_ok = |k: i64, l: i64| (2 * l + 1).pow(2) <= 4 * r.pow(2) - (2 * k + 1).pow(2);

    let find_upper_bound = |k: i64| {
        let mut ok = 0_i64;
        let mut ng = MAX;

        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) / 2;

            if is_ok(k, mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    };

    let mut quarter = 0_i64;
    for k in 1..=r - 1 {
        if is_ok(k, 0) {
            quarter += find_upper_bound(k);
        }
    }
    let mut ans = 4 * quarter;
    if is_ok(0, 0) {
        ans += 1 + 4 * find_upper_bound(0);
    }
    println!("{}", ans);
}
