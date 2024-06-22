use proconio::input;

fn main() {
    input! {
        (mut sx, mut sy): (usize, usize),
        (mut tx, mut ty): (usize, usize),
    }

    if sx > tx {
        std::mem::swap(&mut sx, &mut tx);
        std::mem::swap(&mut sy, &mut ty);
    }

    if (sx + sy) % 2 == 0 {
        sx += 1;
    }

    if (tx + ty) % 2 == 0 {
        tx += 1;
    }

    let dx = tx - sx;
    let dy = sy.abs_diff(ty);
    let ans = dx.saturating_sub(dy) / 2 + dy;
    println!("{}", ans);
}
