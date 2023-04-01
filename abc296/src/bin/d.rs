use proconio::input;

fn main() {
    input! {
        (n, m): (u128, u128),
    }

    let ans = (1..=n)
        .take_while(|a| (a - 1).pow(2) <= m)
        .filter_map(|a| {
            let b = (m + a - 1) / a;

            if b <= n {
                Some(a * b)
            } else {
                None
            }
        })
        .min();

    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
