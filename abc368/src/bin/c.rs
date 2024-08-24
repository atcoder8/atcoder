use proconio::input;

fn main() {
    input! {
        n: usize,
        hh: [usize; n],
    }

    let mut t = 0_usize;
    for &h in &hh {
        let (q, r) = (h / 5, h % 5);
        t += 3 * q;

        let mut rem = r;
        while rem != 0 {
            t += 1;

            if t % 3 == 0 {
                rem = rem.saturating_sub(3);
            } else {
                rem -= 1;
            }
        }
    }

    println!("{}", t);
}
