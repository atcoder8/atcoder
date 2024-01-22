use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut ans = 0;
    for left in 0..n {
        let mut visited = [false; 10];
        let mut candidates = [false; 10];

        for (i, &a) in enumerate(&aa[left..]) {
            if candidates[a] {
                ans += n - (left + i);
                break;
            }

            for left_a in (0..10).filter(|&visited_a| visited[visited_a]) {
                if 2 * a >= left_a && 2 * a - left_a < 10 {
                    candidates[2 * a - left_a] = true;
                }
            }

            visited[a] = true;
        }
    }

    println!("{}", ans);
}
