use itertools::{enumerate, iproduct};
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut dp = vec![[None; 3]; n + 1];
    dp[0] = [Some(0_usize); 3];
    for (i, c) in enumerate(s.chars()) {
        let aoki_hand = hand_to_usize(c);

        for (prev, cur) in iproduct!(0..3, 0..3) {
            if (cur + 1) % 3 == aoki_hand || prev == cur {
                continue;
            }

            let win = (cur + 2) % 3 == aoki_hand;
            if let Some(prev_win_num) = dp[i][prev] {
                chmax_for_option(&mut dp[i + 1][cur], prev_win_num + win as usize);
            }
        }
    }

    let ans = dp[n].iter().filter_map(|&win_num| win_num).max().unwrap();
    println!("{}", ans);
}

fn hand_to_usize(hand: char) -> usize {
    match hand {
        'R' => 0,
        'P' => 1,
        'S' => 2,
        _ => panic!(),
    }
}

/// If `value` is `None` or contains a value less than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmax_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost >= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
