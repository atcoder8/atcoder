fn main() {
    let (n, k): (usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let aa: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let mut xy: Vec<(f64, f64)> = Vec::new();
    for _ in 0..n {
        xy.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        });
    }

    let mut ok: f64 = 2e18;
    let mut ng: f64 = 0.0;
    while (ok - ng).abs() > 1e-6 {
        let mid = (ok + ng) / 2.0;
        let mut flags = vec![false; n];
        for i in 0..k {
            let (x1, y1) = xy[aa[i] - 1];
            for j in 0..n {
                let (x2, y2) = xy[j];
                if (x1 - x2).powi(2) + (y1 - y2).powi(2) <= mid.powi(2) {
                    flags[j] = true;
                }
            }
        }
        if flags.iter().all(|x| *x) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
