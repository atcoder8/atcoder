use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let max_a = *aa.iter().max().unwrap();

    let mut factor_counts = vec![0_usize; max_a + 1];
    for p in 2..=max_a {
        if factor_counts[p] != 0 {
            continue;
        }

        for i in (p..=max_a).step_by(p) {
            let mut divided = i;
            while divided % p == 0 {
                divided /= p;
                factor_counts[i] += 1;
            }
        }
    }

    let grundy = aa.iter().fold(0_usize, |acc, &a| acc ^ factor_counts[a]);
    println!("{}", if grundy != 0 { "Anna" } else { "Bruno" });
}
