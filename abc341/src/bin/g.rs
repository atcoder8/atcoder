use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [f64; n],
    }

    let mut prefix_sum = vec![0.0; n + 1];
    for (i, &a) in enumerate(&aa) {
        prefix_sum[i + 1] = prefix_sum[i] + a;
    }

    let mut ans = vec![0.0; n];
    let mut stack = vec![n];
    for (x, &y) in enumerate(&prefix_sum[..n]).rev() {
        while stack.len() >= 2 {
            let x1 = stack[stack.len() - 2];
            let y1 = prefix_sum[x1];
            let slope1 = (y1 - y) / (x1 - x) as f64;

            let x2 = stack[stack.len() - 1];
            let y2 = prefix_sum[x2];
            let slope2 = (y2 - y) / (x2 - x) as f64;

            if slope1 >= slope2 {
                stack.pop();
            } else {
                break;
            }
        }

        let other_x = *stack.last().unwrap();
        let other_y = prefix_sum[other_x];
        ans[x] = (other_y - y) / (other_x - x) as f64;

        stack.push(x);
    }

    println!("{}", ans.iter().join("\n"));
}
