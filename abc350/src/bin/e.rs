use std::cmp::min_by;

use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        (n, a, x, y): (usize, usize, f64, f64),
    }

    let ans = rec(n, a, x, y, &mut HashMap::new());
    println!("{}", ans);
}

fn rec(n: usize, a: usize, x: f64, y: f64, memo: &mut HashMap<usize, f64>) -> f64 {
    if n == 0 {
        return 0.0;
    }

    if let Some(&cost) = memo.get(&n) {
        return cost;
    }

    let cost1 = x + rec(n / a, a, x, y, memo);
    let cost2 = 1.2 * y + (2..=6).map(|b| rec(n / b, a, x, y, memo)).sum::<f64>() / 5.0;

    let min_cost = min_by(cost1, cost2, |x, y| x.partial_cmp(y).unwrap());
    memo.insert(n, min_cost);

    min_cost
}
