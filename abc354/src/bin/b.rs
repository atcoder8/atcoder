use proconio::input;

fn main() {
    input! {
        n: usize,
        mut sc: [(String, usize); n],
    }

    sc.sort_unstable_by_key(|v| v.0.clone());

    let t = sc.iter().map(|v| v.1).sum::<usize>();
    let ans = &sc[t % n].0;
    println!("{}", ans);
}
