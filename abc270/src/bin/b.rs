use proconio::input;

fn main() {
    if let Some(dist) = solve() {
        println!("{}", dist);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<i64> {
    input! {
        (x, y, z): (i64, i64, i64),
    }

    let mut pairs = vec![(x, 'x'), (y, 'y'), (z, 'z'), (0, 'o')];
    pairs.sort_by_key(|x| x.0);

    let x_pos = pairs.iter().position(|x| x.1 == 'x').unwrap();
    let o_pos = pairs.iter().position(|x| x.1 == 'o').unwrap();
    if x_pos > o_pos {
        pairs.reverse();
    }

    let x_pos = pairs.iter().position(|x| x.1 == 'x').unwrap();
    let y_pos = pairs.iter().position(|x| x.1 == 'y').unwrap();
    let z_pos = pairs.iter().position(|x| x.1 == 'z').unwrap();
    let o_pos = pairs.iter().position(|x| x.1 == 'o').unwrap();

    if x_pos < y_pos && y_pos < o_pos {
        if y_pos < z_pos && z_pos < o_pos {
            Some(x.abs())
        } else if o_pos < z_pos {
            Some(2 * z.abs() + x.abs())
        } else {
            None
        }
    } else {
        Some(x.abs())
    }
}
