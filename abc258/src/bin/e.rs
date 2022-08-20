fn main() {
    let (n, q, x): (usize, usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let ww: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let mut kk: Vec<usize> = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        kk.push(line.trim().parse().unwrap());
    }

    let mut cumsum = vec![0];
    for i in 0..n {
        cumsum.push(cumsum[i] + ww[i]);
    }

    let total_w: usize = cumsum[n];
    let (q, r) = (x / total_w, x % total_w);

    let mut doubling = vec![vec![0; n]; 60];
    let mut potato_num = vec![0; n];

    for i in 0..n {
        if r == 0 {
            doubling[0][i] = i;
            potato_num[i] = q * n;
        } else {
            let mut ok: usize = n;
            let mut ng: usize = 0;
            while ok - ng > 1 {
                let mid = (ok + ng) / 2;
                let sum_w = if i + mid - 1 < n {
                    cumsum[i + mid] - cumsum[i]
                } else {
                    cumsum[n] - cumsum[i] + cumsum[(i + mid) - n]
                };
                if sum_w >= r {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            doubling[0][i] = (i + ok) % n;
            potato_num[i] = q * n + ok;
        }
    }

    for i in 0..59 {
        for j in 0..n {
            doubling[i + 1][j] = doubling[i][doubling[i][j]];
        }
    }

    for k in kk {
        let mut idx = 0;
        let k2 = k - 1;
        for i in 0..60 {
            if (k2 >> i) & 1 == 1 {
                idx = doubling[i][idx];
            }
        }

        println!("{}", potato_num[idx]);
    }
}
