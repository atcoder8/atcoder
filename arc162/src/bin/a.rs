use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
        ppp: [[Usize1]; t],
    }

    for pp in &ppp {
        let mut inv = vec![0; pp.len()];
        for (i, &p) in pp.iter().enumerate() {
            inv[p] = i + 1;
        }

        let mut ans = 0;
        let mut max = 0;
        for &i in &inv {
            if i > max {
                ans += 1;
                max = i;
            }
        }

        println!("{}", ans);
    }
}
