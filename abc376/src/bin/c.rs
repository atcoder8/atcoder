use proconio::input;

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        mut aa: [usize; n],
        mut bb: [usize; n - 1],
    }

    aa.sort_unstable();
    bb.sort_unstable();

    let mut add_size = None;
    for &a in aa.iter().rev() {
        if bb.last().is_some_and(|&b| b >= a) {
            bb.pop();
            continue;
        }

        if add_size.is_some() {
            return None;
        }

        add_size = Some(a);
    }

    add_size
}
