use std::collections::VecDeque;

fn main() {
    proconio::input! {
        n: i64,
    }
    let mut tt = Vec::new();
    let mut aaa = Vec::new();
    for _ in 0..n {
        proconio::input! {
            t: i64,
            k: i64,
            mut aa: [i64; k],
        }
        aa.iter_mut().for_each(|x| *x -= 1);
        tt.push(t);
        aaa.push(aa);
    }

    let mut ans = 0_i64;
    let mut used = vec![false; n as usize];
    let mut deq = VecDeque::new();
    deq.push_back(n - 1);
    while !deq.is_empty() {
        let q = deq.pop_front().unwrap();
        if used[q as usize] {
            continue;
        }
        ans += tt[q as usize];
        used[q as usize] = true;
        for &next in aaa[q as usize].iter() {
            deq.push_back(next);
        }
    }

    println!("{}", ans);
}
