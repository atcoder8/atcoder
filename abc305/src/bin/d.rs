use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut sleep = vec![0];
    for i in 1..n {
        if i % 2 == 0 {
            sleep.push(sleep[i - 1] + aa[i] - aa[i - 1]);
        } else {
            sleep.push(sleep[i - 1]);
        }
    }
    sleep.push(*sleep.last().unwrap());

    for &(l, r) in &lr {
        let left = aa.upper_bound(&l) - 1;
        let right = aa.upper_bound(&r);

        let mut ans = sleep[right] - sleep[left];
        if left % 2 == 1 {
            ans -= l - aa[left];
        }
        if right % 2 == 0 {
            ans -= aa[right] - r;
        }

        println!("{}", ans);
    }
}
