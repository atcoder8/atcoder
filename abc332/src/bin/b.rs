use proconio::input;

fn main() {
    input! {
        (k, g, m): (usize, usize, usize),
    }

    let mut glass = 0;
    let mut mug = 0;
    for _ in 0..k {
        if glass == g {
            glass = 0;
        } else if mug == 0 {
            mug = m;
        } else {
            let rem = g - glass;
            if rem >= mug {
                glass += mug;
                mug = 0;
            } else {
                glass = g;
                mug -= rem;
            }
        }
    }

    println!("{} {}", glass, mug);
}
