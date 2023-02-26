use im_rc::HashSet;
use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut used = HashSet::new();
    used.insert((0, 0));

    let mut x = 0;
    let mut y = 0;

    for &c in &s {
        match c {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => panic!(),
        }

        let coord = (x, y);

        if used.contains(&coord) {
            return true;
        }

        used.insert(coord);
    }

    false
}
