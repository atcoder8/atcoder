use std::collections::VecDeque;

use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w, t): (usize, usize, usize),
        aaa: [Chars; h],
    }

    let mut start_pos = (0, 0);
    let mut goal_pos = (0, 0);
    let mut candies = vec![];

    for i in 0..h {
        for j in 0..w {
            let pos = (i, j);

            match aaa[i][j] {
                'S' => start_pos = pos,
                'G' => goal_pos = pos,
                'o' => candies.push(pos),
                _ => {}
            }
        }
    }

    let candy_num = candies.len();

    candies.push(start_pos);
    candies.push(goal_pos);

    let extended_candy_num = candy_num + 2;

    let mut dists = vec![vec![None; extended_candy_num]; extended_candy_num];

    for candy_idx in 0..extended_candy_num {
        let (candy_x, candy_y) = candies[candy_idx];

        let mut mat = vec![vec![None; w]; h];
        mat[candy_x][candy_y] = Some(0_usize);
        let mut queue = VecDeque::from(vec![(candy_x, candy_y)]);

        while let Some((cur_x, cur_y)) = queue.pop_front() {
            let next_dist = mat[cur_x][cur_y].unwrap() + 1;

            for &(diff_x, diff_y) in &DIFFS {
                let next_x = cur_x.wrapping_add(diff_x);
                let next_y = cur_y.wrapping_add(diff_y);

                if next_x < h
                    && next_y < w
                    && aaa[next_x][next_y] != '#'
                    && mat[next_x][next_y].is_none()
                {
                    mat[next_x][next_y] = Some(next_dist);
                    queue.push_back((next_x, next_y));
                }
            }
        }

        for other_candy_idx in 0..extended_candy_num {
            let (other_candy_x, other_candy_y) = candies[other_candy_idx];
            dists[candy_idx][other_candy_idx] = mat[other_candy_x][other_candy_y];
        }
    }

    let mut dp: Vec<Vec<Option<usize>>> = vec![vec![None; candy_num]; 1_usize << candy_num];
    for i in 0..candy_num {
        dp[1_usize << i][i] = dists[candy_num][i];
    }

    for bit in 0..(1_usize << candy_num) {
        for from in 0..candy_num {
            if (bit >> from) & 1 == 0 {
                continue;
            }

            let dist1 = if let Some(dist) = dp[bit][from] {
                dist
            } else {
                continue;
            };

            for to in 0..candy_num {
                if (bit >> to) & 1 == 1 {
                    continue;
                }

                let dist2 = if let Some(dist) = dists[from][to] {
                    dist
                } else {
                    continue;
                };

                let candidate_dist = dist1 + dist2;
                let next_dist = &mut dp[bit | 1_usize << to][to];
                if next_dist.is_none() || candidate_dist < next_dist.unwrap() {
                    *next_dist = Some(candidate_dist);
                }
            }
        }
    }

    let mut ans = None;
    for bit in 0..(1_usize << candy_num) {
        let obtained_candy_num = bit.count_ones();

        for last_candy in 0..candy_num {
            if let (Some(body_dist), Some(tail_dist)) =
                (dp[bit][last_candy], dists[last_candy][candy_num + 1])
            {
                let candidate_dist = body_dist + tail_dist;
                if candidate_dist <= t && (ans.is_none() || obtained_candy_num > ans.unwrap()) {
                    ans = Some(obtained_candy_num);
                }
            }
        }
    }
    let min_dist = dists[candy_num][candy_num + 1];
    if ans.is_none() && min_dist.is_some() && min_dist.unwrap() <= t {
        ans = Some(0);
    }

    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
