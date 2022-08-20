use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        (n, k): (usize, i64),
        mut aa: [i64; n],
    }

    let min_a = *aa.iter().min().unwrap();
    aa.iter_mut().for_each(|x| *x -= min_a);

    let mut ans = 0;
    let mut deq = VecDeque::new();
    let mut sum = 0;
    for a in aa {
        deq.push_back(a);
        sum += a;
        while !deq.is_empty() && sum + min_a * deq.len() as i64 > k {
            let rm = deq.pop_front().unwrap();
            sum -= rm;
        }
        println!("sum = {}", sum);
        if sum + min_a * deq.len() as i64 == k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
