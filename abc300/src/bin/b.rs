use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (h, w): (usize, usize),
        mut aaa: [Chars; h],
        bbb: [Chars; h],
    }

    let is_match = |shift_x: usize, shift_y: usize| {
        for i in 0..h {
            for j in 0..w {
                if aaa[(i + shift_x) % h][(j + shift_y) % w] != bbb[i][j] {
                    return false;
                }
            }
        }

        true
    };

    for shift_x in 0..h {
        for shift_y in 0..w {
            if is_match(shift_x, shift_y) {
                return true;
            }
        }
    }

    false
}
