use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut ab: [(usize, usize); n],
    }

    ab.sort_unstable_by_key(|x| x.0);
    let mut sum: usize = ab.iter().map(|x| x.1).sum();

    if sum <= k {
        println!("1");
        std::process::exit(0);
    }

    for &(a, b) in &ab {
        sum -= b;

        if sum <= k {
            println!("{}", a + 1);
            break;
        }
    }
}
