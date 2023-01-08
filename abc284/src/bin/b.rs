use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            aa: [usize; n],
        }

        let ans = aa.iter().filter(|&&a| a % 2 == 1).count();
        println!("{}", ans);
    }
}
