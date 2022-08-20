use proconio::input;

fn main() {
    input! {
        (n, mut x): (usize, usize),
        mut aa: [usize; n],
    }
    x -= 1;
    aa.iter_mut().for_each(|x| *x -= 1);

    let mut used = vec![false; n];
    let mut ans = 0;
    while !used[x] {
        ans += 1;
        used[x] = true;
        x = aa[x];
    }

    println!("{}", ans);
}
