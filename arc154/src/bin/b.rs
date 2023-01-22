use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    if let Some(ans) = solve() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    let mut sorted_s = s.clone();
    sorted_s.sort_unstable();

    let mut sorted_t = t.clone();
    sorted_t.sort_unstable();

    if sorted_s != sorted_t {
        return None;
    }

    let ss = s.iter().map(|&c| (c as u8 - b'a') as usize).collect_vec();
    let tt = t.iter().map(|&c| (c as u8 - b'a') as usize).collect_vec();

    let mut progress = 0;
    let mut stock = vec![0; 26];
    let mut s_idx = n - 1;
    let mut t_idx = n - 1;

    while s_idx > progress {
        if ss[s_idx] == tt[t_idx] {
            s_idx -= 1;
            t_idx -= 1;
        } else if stock[tt[t_idx]] >= 1 {
            stock[tt[t_idx]] -= 1;
            t_idx -= 1;
        } else {
            while ss[progress] != tt[t_idx] {
                stock[ss[progress]] += 1;
                progress += 1;
            }

            t_idx -= 1;
            progress += 1;
        }
    }

    Some(progress)
}
