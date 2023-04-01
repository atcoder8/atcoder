use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        (n, x): (usize, i64),
        aa: [i64; n],
    }

    let set: HashSet<i64> = aa.iter().cloned().collect();

    let ans = aa.iter().any(|&a| set.contains(&(a - x)));
    println!("{}", if ans { "Yes" } else { "No" });
}
