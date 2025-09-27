use itertools::{enumerate, iproduct, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> u32 {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
    }

    let raised = 2_usize.pow(w as u32);

    let mut expanded_ss = ss.clone();
    expanded_ss.insert(0, vec!['.'; w]);

    let mut dp = vec![None::<u32>; raised];
    dp[0] = Some(0);
    let mut next_dp = vec![None::<u32>; raised];

    for (s1, s2) in ss.iter().tuple_windows() {
        for (bits, &cnt) in enumerate(&dp) {
            let Some(cnt) = cnt else {
                continue;
            };

            let painted1 = enumerate(s1)
                .map(|(i, &c)| c == '#' && (bits >> i & 1 == 0))
                .collect_vec();

            for next_bits in 0..raised {
                let add_cnt = next_bits.count_ones();
                let painted2 = enumerate(s2)
                    .map(|(i, &c)| c == '#' && (next_bits >> i & 1 == 0))
                    .collect_vec();

                let sub_grid = [&painted1, &painted2];
                if (0..w - 1)
                    .all(|col| iproduct!(0..2, 0..2).any(|(dr, dc)| !sub_grid[dr][col + dc]))
                {
                    chmin_for_option(&mut next_dp[next_bits], cnt + add_cnt);
                }
            }
        }

        std::mem::swap(&mut dp, &mut next_dp);
        next_dp.fill(None);
    }

    dp.iter().filter_map(|&v| v).min().unwrap()
}

/// If `value` is `None` or contains a value greater than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmin_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost <= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
