use proconio::input;
use superslice::Ext;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        n: usize,
        s: String,
        t: String,
    }

    let mut indexes_each_char = vec![vec![]; 26];
    for (i, c) in s.chars().enumerate() {
        indexes_each_char[char_to_number(c)].push(i);
    }

    if t.chars()
        .any(|c| indexes_each_char[char_to_number(c)].is_empty())
    {
        return 0;
    }

    let is_ok = |k: usize| {
        if k == 0 {
            return true;
        }

        let mut cycle = 0;
        let mut idx = 0;

        for c in t.chars() {
            let indexes = &indexes_each_char[char_to_number(c)];
            let char_num = indexes.len();

            let mut start = indexes.lower_bound(&idx);
            if start == char_num {
                cycle += 1;
                start = 0;
            }

            cycle += (start + k - 1) / char_num;
            idx = indexes[(start + k - 1) % char_num] + 1;

            if idx == s.len() {
                idx = 0;
                cycle += 1;
            }

            if cycle >= n {
                return false;
            }
        }

        true
    };

    let mut ok = 0_usize;
    let mut ng = n * s.len() / t.len() + 1;
    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    ok
}

/// Converts the character `c` to the corresponding numeric value.
pub fn char_to_number(c: char) -> usize {
    (c as u8 - b'a') as usize
}
