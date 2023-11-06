use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        abw: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, w) in &abw {
        graph[a].push((b, w));
        graph[b].push((a, w));
    }

    let is_ok = |x: usize| {
        // Red: `Some(false)`, Blue: `Some(true)`, Undecided: `None`
        let mut colors: Vec<Option<bool>> = vec![None; n];
        let mut weights = vec![vec![]; n];
        for start in 0..n {
            if colors[start].is_some() {
                continue;
            }

            let mut stack = vec![(start, false)];
            while let Some((cur, color)) = stack.pop() {
                if let Some(decided_color) = colors[cur] {
                    if decided_color != color {
                        return false;
                    }

                    continue;
                }

                colors[cur] = Some(color);

                for &(next, w) in &graph[cur] {
                    if w < x {
                        weights[cur].push(w);
                        stack.push((next, !color));
                    }
                }
            }
        }

        weights.iter_mut().for_each(|ws| ws.sort_unstable());

        weights.iter().all(|ws| ws.len() < 2 || ws[0] + ws[1] >= x)
    };

    let mut ok = 0;
    let mut ng = 2 * 10_usize.pow(9) + 1;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
