use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        xy: [(i64, i64); n],
        pq: [(i64, i64); m],
    }

    let mut dp = vec![vec![vec![2e18; m + 1]; n + m]; 1_usize << (n + m)];

    for (i, &(x, y)) in xy.iter().enumerate() {
        dp[1_usize << i][i][0] = ((x.pow(2) + y.pow(2)) as f64).sqrt();
    }

    for (i, &(p, q)) in pq.iter().enumerate() {
        dp[1_usize << (i + n)][i + n][1] = ((p.pow(2) + q.pow(2)) as f64).sqrt();
    }

    for bit in 0..(1_usize << (n + m)) {
        for source in 0..(n + m) {
            if (bit >> source) & 1 == 0 {
                continue;
            }

            for dest in 0..(n + m) {
                if (bit >> dest) & 1 == 1 {
                    continue;
                }

                for boost in 0..=m {
                    if boost == m && dest >= n {
                        continue;
                    }

                    chmin!(
                        dp[bit | (1_usize << dest)][dest][boost + if dest >= n { 1 } else { 0 }],
                        dp[bit][source][boost]
                            + calc_dist(&xy, &pq, source, dest) / (1_usize << boost) as f64
                    );
                }
            }
        }
    }

    let mut ans = 2e18;
    for treasure_bit in 0..(1_usize << m) {
        for locate in 0..(n + m) {
            for boost in 0..=m {
                chmin!(
                    ans,
                    dp[(1 << n) - 1 + (treasure_bit << n)][locate][boost] + {
                        let (x, y) = get_coord(&xy, &pq, locate);
                        ((x.pow(2) + y.pow(2)) as f64).sqrt() / (1_usize << boost) as f64
                    }
                );
            }
        }
    }

    println!("{}", ans);
}

fn get_coord(xy: &Vec<(i64, i64)>, pq: &Vec<(i64, i64)>, locate: usize) -> (i64, i64) {
    if locate < xy.len() {
        xy[locate]
    } else {
        pq[locate - xy.len()]
    }
}

fn calc_dist(xy: &Vec<(i64, i64)>, pq: &Vec<(i64, i64)>, source: usize, dest: usize) -> f64 {
    let (source_x, source_y) = get_coord(xy, pq, source);
    let (dest_x, dest_y) = get_coord(xy, pq, dest);

    (((source_x - dest_x).pow(2) + (source_y - dest_y).pow(2)) as f64).sqrt()
}

/// If the right-hand side is less than the left-hand side,
/// clones the right-hand side, bind it to the left-hand side,
/// and return `true`.
/// If the right-hand side is greater than or equal to the left-hand side,
/// does nothing and returns `false`.
///
/// The left-hand and right-hand sides must be the same type
/// and must implement the `Clone` trait.
///
/// # Examples
///
/// ```
/// let mut x = 5;
///
/// assert_eq!(chmin!(x, 7), false);
/// assert_eq!(x, 5);
///
/// assert_eq!(chmin!(x, 3), true);
/// assert_eq!(x, 3);
/// ```
#[macro_export]
macro_rules! chmin {
    ($lhs: expr, $rhs: expr) => {
        if $rhs < $lhs {
            let temp = $rhs.clone();
            $lhs = temp;
            true
        } else {
            false
        }
    };
}
