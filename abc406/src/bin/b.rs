use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let ans = aa.iter().fold(1_usize, |curr, &a| {
        let next = curr.saturating_mul(a);
        if next.to_string().len() <= k {
            next
        } else {
            1
        }
    });
    println!("{}", ans);
}
