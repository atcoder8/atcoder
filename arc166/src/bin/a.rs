use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
        nxy: [(usize, Chars, Chars); t],
    }

    for (_, xx, yy) in nxy {
        println!("{}", if solve(xx, yy) { "Yes" } else { "No" });
    }
}

fn solve(mut xx: Vec<char>, yy: Vec<char>) -> bool {
    let n = xx.len();

    if xx.iter().zip(&yy).any(|(&x, &y)| y == 'C' && x != 'C') {
        return false;
    }

    let mut sections = vec![];
    let mut left = 0;
    while left < n {
        let right = (left..n).find(|&right| yy[right] == 'C').unwrap_or(n);
        if left < right {
            sections.push((left, right));
        }
        left = right + 1;
    }

    for (left, right) in sections {
        let xa = xx[left..right].iter().filter(|&&c| c == 'A').count();
        let ya = yy[left..right].iter().filter(|&&c| c == 'A').count();

        if xa > ya {
            return false;
        }

        let mut rem = ya - xa;
        for c in xx[left..right].iter_mut() {
            if *c == 'C' {
                if rem != 0 {
                    *c = 'A';
                    rem -= 1;
                } else {
                    *c = 'B';
                }
            }
        }

        if rem != 0 {
            return false;
        }

        let mut a = 0;
        for i in left..right {
            if xx[i] == 'B' && yy[i] == 'A' {
                if a != 0 {
                    a -= 1;
                } else {
                    return false;
                }
            } else if xx[i] == 'A' && yy[i] == 'B' {
                a += 1;
            }
        }
    }

    true
}
