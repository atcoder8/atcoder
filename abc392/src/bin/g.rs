use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ss: [usize; n],
    }

    ss.sort_unstable();

    let max_s = *ss.iter().max().unwrap();
    let mut counts = vec![0_i64; max_s + 1];
    for &s in &ss {
        counts[s] += 1;
    }

    let convolution = ac_library::convolution_i64(&counts, &counts);

    let mut ans = 0_i64;
    for i in 0..=max_s {
        if counts[i] != 0 {
            ans += convolution[2 * i] - 1;
        }
    }
    ans /= 2;

    println!("{}", ans);
}
