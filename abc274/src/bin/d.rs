use itertools::Itertools;
use proconio::input;

const JACK_UP: usize = 10_000;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, x, y): (usize, i64, i64),
        aa: [usize; n],
    }

    let dist_xs = aa.iter().cloned().step_by(2).collect_vec();
    let dist_ys = aa.iter().cloned().skip(1).step_by(2).collect_vec();

    solve_sub(&dist_xs, x, true) && solve_sub(&dist_ys, y, false)
}

fn solve_sub(distances: &Vec<usize>, dest: i64, x_dir: bool) -> bool {
    let mut dp = vec![false; 2 * JACK_UP + 1];
    if x_dir {
        dp[distances[0] + JACK_UP] = true;
    } else {
        dp[JACK_UP] = true;
    }

    for &dist in distances.iter().skip(if x_dir { 1 } else { 0 }) {
        let mut next_dp = vec![false; 2 * JACK_UP + 1];

        for (cur_coord, _) in dp.iter().enumerate().filter(|(_, &flag)| flag) {
            if cur_coord >= dist {
                next_dp[cur_coord - dist] = true;
            }

            if cur_coord + dist <= 2 * JACK_UP {
                next_dp[cur_coord + dist] = true;
            }
        }

        dp = next_dp;
    }

    dp[(dest + JACK_UP as i64) as usize]
}
