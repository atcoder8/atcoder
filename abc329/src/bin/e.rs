use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        s: Chars,
        t: Chars,
    }

    let mut complete = vec![false; n + 1];
    let mut partial = vec![false; n + 1];
    partial[0] = true;
    for i in 0..n {
        if !complete[i] && !partial[i] {
            continue;
        }

        for extend_len in 1..=m.min(n - i) {
            let overlap_len = m - extend_len;

            if overlap_len > i || !complete[i] && s[i - overlap_len..i] != t[..overlap_len] {
                continue;
            }

            let match_len = (0..extend_len)
                .take_while(|&j| s[i + j] == t[overlap_len + j])
                .count();
            if match_len == extend_len {
                complete[i + extend_len] = true;
            } else {
                partial[i + match_len] = true;
            }
        }
    }

    complete[n]
}
