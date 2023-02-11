use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, k): (usize, usize),
        mut ss: [Chars; n],
    }

    ss[..k].sort();

    for s in ss.iter().take(k) {
        let s: String = s.iter().collect();
        println!("{}", s);
    }
}
