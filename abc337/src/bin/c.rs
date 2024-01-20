use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut par_to_child = vec![None; n];
    for (child, &par) in enumerate(&aa) {
        if par != -1 {
            par_to_child[par as usize - 1] = Some(child);
        }
    }
    let head = aa.iter().position(|&a| a == -1).unwrap();

    let mut seq = vec![head + 1];
    seq.reserve(n);
    let mut cur = head;
    while let Some(next) = par_to_child[cur] {
        seq.push(next + 1);
        cur = next;
    }

    println!("{}", seq.iter().join(" "));
}
