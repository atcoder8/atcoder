use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [[usize]; m],
    }

    let mut ans = 0;

    for bit in 0..(1_usize << m) {
        let mut flags = vec![false; n];

        for i in 0..m {
            if (bit >> i) & 1 == 1 {
                for &a in &ss[i] {
                    flags[a - 1] = true;
                }
            }
        }

        if flags.iter().all(|&flag| flag) {
            ans += 1;
        }
    }

    println!("{}", ans);
}
