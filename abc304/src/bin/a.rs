use proconio::input;

fn main() {
    input! {
        n: usize,
        sa: [(String, usize); n],
    }

    let min_pos = (0..n).min_by_key(|&i| sa[i].1).unwrap();
    for i in 0..n {
        println!("{}", sa[(min_pos + i) % n].0);
    }
}
