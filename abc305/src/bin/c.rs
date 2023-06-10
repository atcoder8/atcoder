use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        sss: [Chars; h],
    }

    let top = (0..h).find(|&i| sss[i].iter().any(|&s| s == '#')).unwrap();
    let bottom = (0..h)
        .rev()
        .find(|&i| sss[i].iter().any(|&s| s == '#'))
        .unwrap();
    let left = (0..w).find(|&j| (0..h).any(|i| sss[i][j] == '#')).unwrap();
    let right = (0..w)
        .rev()
        .find(|&j| (0..h).any(|i| sss[i][j] == '#'))
        .unwrap();

    for i in top..=bottom {
        for j in left..=right {
            if sss[i][j] == '.' {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
