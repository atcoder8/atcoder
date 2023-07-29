use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
    }

    let is_code = |top: usize, left: usize| {
        for i in 0..3 {
            for j in 0..3 {
                if ss[top + i][left + j] != '#' || ss[top + 6 + i][left + 6 + j] != '#' {
                    return false;
                }
            }
        }

        for i in 0..4 {
            if ss[top + i][left + 3] != '.' || ss[top + 5 + i][left + 5] != '.' {
                return false;
            }
        }

        for i in 0..3 {
            if ss[top + 3][left + i] != '.' || ss[top + 5][left + 6 + i] != '.' {
                return false;
            }
        }

        true
    };

    for top in 0..=(n - 9) {
        for left in 0..=(m - 9) {
            if is_code(top, left) {
                println!("{} {}", top + 1, left + 1);
            }
        }
    }
}
