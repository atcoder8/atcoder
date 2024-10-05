use proconio::input;

fn main() {
    input! {
        n: usize,
        kk: [usize; n],
    }

    let sum_k = kk.iter().sum::<usize>();

    let find_max = |bits: u32| {
        let sum = (0..n)
            .filter_map(|i| {
                if bits >> i & 1 == 1 {
                    Some(kk[i])
                } else {
                    None
                }
            })
            .sum::<usize>();
        sum.max(sum_k - sum)
    };

    let ans = (0..1 << n).map(find_max).min().unwrap();
    println!("{}", ans);
}
