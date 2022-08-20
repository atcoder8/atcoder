fn main() {
    let (_n, a, b): (usize, usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
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

    let mut ok: usize = *aa.iter().min().unwrap();
    let mut ng: usize = *aa.iter().max().unwrap() + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        let (mut minus_cnt, mut plus_cnt) = (0, 0);
        aa.iter().for_each(|x| {
            if *x < mid {
                minus_cnt += (mid - x + a - 1) / a;
            } else {
                plus_cnt += (x - mid) / b;
            }
        });

        if plus_cnt >= minus_cnt {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
