use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }

    let mut ans = 0;
    for i in 0..100 {
        for j in 0..100 {
            ans += abcd
                .iter()
                .any(|&(a, b, c, d)| a <= i && i < b && c <= j && j < d)
                as usize;
        }
    }
    println!("{}", ans);
}
