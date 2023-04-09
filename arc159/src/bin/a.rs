use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, _k): (usize, usize),
        aaa: [[usize; n]; n],
        q: usize,
        st: [(Usize1, Usize1); q],
    }

    let mut distances: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];
    for i in 0..n {
        for j in 0..n {
            if aaa[i][j] == 1 {
                distances[i][j] = Some(1);
            }
        }
    }
    for mid in 0..n {
        for from in 0..n {
            for to in 0..n {
                if let (Some(dist1), Some(dist2)) = (distances[from][mid], distances[mid][to]) {
                    let candidate_dist = dist1 + dist2;
                    let dist = &mut distances[from][to];

                    if dist.is_none() || candidate_dist < dist.unwrap() {
                        *dist = Some(candidate_dist);
                    }
                }
            }
        }
    }

    for &(s, t) in &st {
        if let Some(dist) = distances[s % n][t % n] {
            println!("{}", dist);
        } else {
            println!("-1");
        }
    }
}
