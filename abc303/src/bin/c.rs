use im_rc::HashSet;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (_n, m, h, k): (usize, usize, i64, i64),
        s: Chars,
        xy: [(i64, i64); m],
    }

    let mut ans = true;
    let mut items: HashSet<(i64, i64)> = xy.iter().cloned().collect();
    let mut hp = h;
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

        hp -= 1;
        if hp < 0 {
            ans = false;
            break;
        }

        if hp < k && items.contains(&(x, y)) {
            hp = k;
            items.remove(&(x, y));
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
