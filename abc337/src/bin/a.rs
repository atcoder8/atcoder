use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    let mut score1 = 0;
    let mut score2 = 0;
    for &(x, y) in &xy {
        score1 += x;
        score2 += y;
    }

    let ans = match score1.cmp(&score2) {
        std::cmp::Ordering::Less => "Aoki",
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Takahashi",
    };
    println!("{}", ans);
}
