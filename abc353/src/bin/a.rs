use proconio::input;

fn main() {
    input! {
        n: usize,
        hh: [usize; n],
    }

    match hh.iter().position(|&h| h > hh[0]) {
        Some(pos) => println!("{}", pos + 1),
        None => println!("-1"),
    }
}
