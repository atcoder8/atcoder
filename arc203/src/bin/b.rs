// unfinished

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    println!(
        "{}",
        (0..t)
            .map(|_| if solve() { "Yes" } else { "No" })
            .join("\n")
    );
}

fn solve() -> bool {
    input! {
        n: usize,
        aa: [u8; n],
        bb: [u8; n],
    }

    if aa == bb {
        return true;
    }

    if aa.iter().filter(|&&a| a == 1).count() != bb.iter().filter(|&&b| b == 1).count() {
        return false;
    }

    if aa.iter().dedup().collect_vec() == bb.iter().dedup().collect_vec() {
        return true;
    }

    find(&aa) && find(&bb)
}

fn find(seq: &[u8]) -> bool {
    let mut found = [false; 2];
    let mut idx = 0_usize;
    while idx + 1 < seq.len() {
        if !found[0] && &seq[idx..idx + 2] == &[0, 1] {
            found[0] = true;
            idx += 2;
        } else if !found[1] && &seq[idx..idx + 2] == &[1, 0] {
            found[1] = true;
            idx += 2;
        } else {
            idx += 1;
        }
    }

    found[0] && found[1]
}
