use proconio::input;

fn main() {
    input! {
        m: usize,
        dd: [usize; m],
    }

    let sum: usize = dd.iter().sum();
    let mut rem = sum / 2;
    for (i, &d) in dd.iter().enumerate() {
        if rem < d {
            println!("{} {}", i + 1, rem + 1);
            break;
        }

        rem -= d;
    }
}
