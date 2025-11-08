use proconio::input;

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        mut hh: [u64; n],
        mut bb: [u64; m],
    }

    hh.sort_unstable();
    bb.sort_unstable();

    let mut b_iter = bb.iter();
    let cnt = hh.iter().take_while(|h| b_iter.any(|b| b >= h)).count();
    println!("{}", if cnt >= k { "Yes" } else { "No" });
}
