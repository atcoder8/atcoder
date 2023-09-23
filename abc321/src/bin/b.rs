use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        aa: [usize; n - 1],
    }

    let is_ok = |last: usize| {
        let mut scores = aa.clone();
        scores.push(last);
        scores.sort_unstable();

        let sum: usize = scores[1..(n - 1)].iter().sum();
        sum >= x
    };

    let ans = (0..=100).find(|&last| is_ok(last));
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
