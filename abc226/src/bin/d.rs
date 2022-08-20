use std::collections::HashSet;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn main() {
    proconio::input! {
        n: i64,
        xy: [(i64, i64); n],
    };

    let mut magic = HashSet::new();
    for i in 0..n {
        let (x1, y1) = xy[i as usize];
        for j in 0..n {
            if i == j {
                continue;
            }
            let (x2, y2) = xy[j as usize];
            let diff_x = x2 - x1;
            let diff_y = y2 - y1;
            let g = gcd(diff_x, diff_y);
            magic.insert((diff_x / g, diff_y / g));
        }
    }

    println!("{}", magic.len());
}
