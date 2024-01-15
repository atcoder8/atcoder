use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa[0] = 1;
    for i in 1..n {
        aa[i] = aa[i].min(aa[i - 1] + 1);
    }

    aa[n - 1] = 1;
    for i in (0..n - 1).rev() {
        aa[i] = aa[i].min(aa[i + 1] + 1);
    }

    let ans = *aa.iter().max().unwrap();
    println!("{}", ans);
}
