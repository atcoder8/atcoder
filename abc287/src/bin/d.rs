use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let check_match =
        |s_idx: usize, t_idx: usize| s[s_idx] == '?' || t[t_idx] == '?' || s[s_idx] == t[t_idx];

    let diff_len = s.len() - t.len();
    let mut mismatch_num = (0..t.len())
        .filter(|&i| !check_match(diff_len + i, i))
        .count();

    println!("{}", if mismatch_num == 0 { "Yes" } else { "No" });

    for i in 0..t.len() {
        if !check_match(i, i) {
            mismatch_num += 1;
        }

        if !check_match(diff_len + i, i) {
            mismatch_num -= 1;
        }

        println!("{}", if mismatch_num == 0 { "Yes" } else { "No" });
    }
}
