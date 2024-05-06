use ac_library::{convolution, ModInt998244353};
use proconio::input;

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let ff = rec(&aa);

    let sum = aa.iter().sum::<usize>();
    let mut comb = Mint::new(1);

    let mut ans = Mint::new(0);
    for k in 1..=n + 1 {
        ans += ff[k - 1] / comb;
        comb = comb * (sum + 1 - k) / k;
    }

    println!("{}", ans);
}

fn rec(aa: &[usize]) -> Vec<Mint> {
    if aa.len() == 1 {
        return vec![Mint::new(1), Mint::new(aa[0])];
    }

    let mid = aa.len() / 2;
    convolution(&rec(&aa[..mid]), &rec(&aa[mid..]))
}
