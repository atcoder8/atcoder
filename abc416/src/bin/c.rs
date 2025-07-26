use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k, x): (usize, usize, Usize1),
        ss: [String; n],
    }

    let mut stack = vec![String::new()];
    for _ in 0..k {
        stack = stack
            .iter()
            .flat_map(|concat| ss.iter().map(|s| concat.clone() + s))
            .collect();
    }
    stack.sort_unstable();
    println!("{}", stack[x]);
}
