use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        ss: [Chars; n],
        q: usize,
        uv: [(Usize1, Usize1); q],
    }

    let mut distances: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                distances[i][i] = Some((0, 0));
            } else if ss[i][j] == 'Y' {
                distances[i][j] = Some((1, aa[j]));
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if let (Some((dist1, val1)), Some((dist2, val2))) =
                    (distances[j][i], distances[i][k])
                {
                    let next_dist = dist1 + dist2;
                    let next_val = val1 + val2;

                    if let Some((d, v)) = &mut distances[j][k] {
                        if next_dist < *d || (next_dist == *d && next_val > *v) {
                            *d = next_dist;
                            *v = next_val;
                        }
                    } else {
                        distances[j][k] = Some((next_dist, next_val));
                    }
                }
            }
        }
    }

    for &(u, v) in &uv {
        if let Some((cnt, sum_value)) = distances[u][v] {
            println!("{} {}", cnt, aa[u] + sum_value);
        } else {
            println!("Impossible");
        }
    }
}
