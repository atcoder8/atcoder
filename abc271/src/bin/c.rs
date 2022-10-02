use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut aa: [Usize1; n],
    }

    aa.sort_unstable();
    aa.dedup();

    aa.resize(n, 1_000_000_000_000);

    let mut idx = 0;

    let mut ans = 0;

    while idx < aa.len() {
        if aa[idx] != ans {
            if aa.len() >= idx + 2 {
                aa.pop();
                aa.pop();
            } else {
                break;
            }
        } else {
            idx += 1;
        }

        ans += 1;
    }

    println!("{}", ans);
}
