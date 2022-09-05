use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let mut ans = 0;
    let mut deq = VecDeque::from(aa);

    while deq.len() >= 2 {
        let &min_a = deq.front().unwrap();
        let max_a = deq.pop_back().unwrap();

        let r = max_a % min_a;
        if r != 0 {
            deq.push_front(r);
        }

        ans += 1;
    }

    println!("{}", ans);
}
