use std::collections::VecDeque;

fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };
    let (sx, sy, tx, ty): (i64, i64, i64, i64) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let mut xyr: Vec<(i64, i64, i64)> = Vec::new();
    for _ in 0..n {
        xyr.push({
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

    let mut start_circle = n;
    for i in 0..n {
        let (x, y, r) = xyr[i];
        let (diff_x, diff_y) = (x - sx, y - sy);
        if diff_x.pow(2) + diff_y.pow(2) == r.pow(2) {
            start_circle = i;
            break;
        }
    }

    let mut terminal_circle = n;
    for i in 0..n {
        let (x, y, r) = xyr[i];
        let (diff_x, diff_y) = (x - tx, y - ty);
        if diff_x.pow(2) + diff_y.pow(2) == r.pow(2) {
            terminal_circle = i;
            break;
        }
    }

    let mut used = vec![false; n];
    used[start_circle] = true;

    let mut deq = VecDeque::new();
    deq.push_back(start_circle);

    while !deq.is_empty() {
        let curr_idx = deq.pop_front().unwrap();

        if curr_idx == terminal_circle {
            println!("Yes");
            std::process::exit(0);
        }

        let (x1, y1, r1) = xyr[curr_idx];

        for next_idx in 0..n {
            if used[next_idx] {
                continue;
            }

            let (x2, y2, r2) = xyr[next_idx];

            let (diff_x, diff_y) = (x2 - x1, y2 - y1);
            let sq_diff = diff_x.pow(2) + diff_y.pow(2);

            if (r1 - r2).pow(2) <= sq_diff && sq_diff <= (r1 + r2).pow(2) {
                used[next_idx] = true;
                deq.push_back(next_idx);
            }
        }
    }

    println!("No");
}
