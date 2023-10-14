use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        (n, t): (usize, Chars),
        ss: [Chars; n],
    }

    let mut pre = vec![];
    for s in &ss {
        let mut pos = 0;
        for &c in s {
            if c == t[pos] {
                pos += 1;

                if pos == t.len() {
                    break;
                }
            }
        }

        pre.push(pos);
    }

    let mut suf = vec![];
    for s in &ss {
        let mut pos = t.len();
        for &c in s.iter().rev() {
            if c == t[pos - 1] {
                pos -= 1;

                if pos == 0 {
                    break;
                }
            }
        }

        suf.push(pos);
    }
    suf.sort_unstable();

    let ans: usize = pre.iter().map(|&x| suf.lower_bound(&(x + 1))).sum();
    println!("{}", ans);
}
