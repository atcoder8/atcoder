use proconio::input;

trait BinarySearch<T: Ord> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}

impl<T: Ord> BinarySearch<T> for Vec<T> {
    fn lower_bound(&self, x: T) -> usize {
        let mut ok: i64 = self.len() as i64;
        let mut ng: i64 = -1;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if self[mid as usize] >= x {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }

    fn upper_bound(&self, x: T) -> usize {
        let mut ok: i64 = self.len() as i64;
        let mut ng: i64 = -1;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if self[mid as usize] > x {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}

fn main() {
    input! {
        (n, k): (usize, usize),
    }
    let mut pp = vec![0_i64; n];
    for i in 0..n {
        input! {
            (p1, p2, p3): (i64, i64, i64),
        }
        pp[i] = p1 + p2 + p3;
    }

    let mut sorted_pp = pp.clone();
    sorted_pp.sort_unstable();
    for i in 0..n {
        let idx = sorted_pp.upper_bound(pp[i] + 300);
        println!("{}", if n - idx + 1 <= k { "Yes" } else { "No" });
    }
}
