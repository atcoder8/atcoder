use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        mut aaa: [[usize; n]; n],
        bbb: [[usize; n]; n],
    }

    let rotate = |mat: &Vec<Vec<usize>>| -> Vec<Vec<usize>> {
        (0..n)
            .map(|i| (0..n).map(|j| mat[n - 1 - j][i]).collect())
            .collect()
    };

    for _ in 0..4 {
        if (0..n).all(|i| (0..n).all(|j| aaa[i][j] == 0 || bbb[i][j] == 1)) {
            return true;
        }
        aaa = rotate(&aaa);
    }

    false
}
