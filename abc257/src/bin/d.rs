use std::collections::VecDeque;

fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };
    let mut xyp: Vec<(i128, i128, usize)> = Vec::new();
    for _ in 0..n {
        xyp.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        });
    }

    let mut ok = (10_i128).pow(10) as i128;
    let mut ng = -1;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        let mut flag = false;
        for start_idx in 0..n {
            let mut reachable = vec![false; n];
            let mut que = VecDeque::new();
            reachable[start_idx] = true;
            que.push_back(start_idx);
            while let Some(curr_idx) = que.pop_front() {
                let (curr_x, curr_y, curr_p) = xyp[curr_idx];
                for next_idx in 0..n {
                    if reachable[next_idx] {
                        continue;
                    }
                    let (next_x, next_y, _) = xyp[next_idx];
                    let diff_x = (curr_x - next_x).abs();
                    let diff_y = (curr_y - next_y).abs();
                    if curr_p as i128 * mid >= diff_x + diff_y {
                        reachable[next_idx] = true;
                        que.push_back(next_idx);
                    }
                }
            }
            if reachable.iter().all(|x| *x) {
                flag = true;
                break;
            }
        }
        if flag { ok = mid } else { ng = mid };
    }

    println!("{}", ok);
}
