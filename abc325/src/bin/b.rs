use proconio::input;

fn main() {
    input! {
        n: usize,
        wx: [(usize, usize); n],
    }

    let mut seq = vec![0; 24];
    for &(w, x) in &wx {
        seq[x] += w;
    }

    let mut ans = 0;
    for start in 0..24 {
        ans = ans.max((0..9).map(|i| seq[(start + i) % 24]).sum());
    }
    println!("{}", ans);
}
