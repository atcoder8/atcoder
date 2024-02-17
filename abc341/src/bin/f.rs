use itertools::izip;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
        ww: [usize; n],
        mut aa: [usize; n],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut memo = vec![None; n];
    for i in 0..n {
        rec(&graph, &ww, &aa, i, &mut memo);
    }

    let ans = izip!(aa, memo)
        .map(|(a, cnt)| a * cnt.unwrap())
        .sum::<usize>();
    println!("{}", ans);
}

fn rec(
    graph: &[Vec<usize>],
    ww: &[usize],
    aa: &[usize],
    cur: usize,
    memo: &mut [Option<usize>],
) -> usize {
    if let Some(cnt) = memo[cur] {
        return cnt;
    }

    let mut dp = vec![0; ww[cur]];
    for &next in &graph[cur] {
        if ww[next] >= ww[cur] {
            continue;
        }

        let mut next_dp = dp.clone();

        for next_sum in ww[next]..ww[cur] {
            next_dp[next_sum] =
                next_dp[next_sum].max(dp[next_sum - ww[next]] + rec(graph, ww, aa, next, memo));
        }

        dp = next_dp;
    }

    let cnt = *dp.iter().max().unwrap() + 1;

    memo[cur] = Some(cnt);

    cnt
}
