use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut ans = 0;

    for &a in &aa {
        let mut a = a;
        while a != 0 {
            ans += a % 10;
            a /= 10;
        }
    }
    ans *= 2 * n;

    let mut carry_up_cnt = 0;
    for i in 0..15 {
        let ten = 10_usize.pow(i + 1);

        let mut bb = aa.iter().map(|&a| a % ten).collect_vec();
        bb.sort_unstable();

        for &a in &aa {
            carry_up_cnt += n - bb.lower_bound(&(ten - a % ten));
        }
    }
    ans -= 9 * carry_up_cnt;

    println!("{}", ans);
}
