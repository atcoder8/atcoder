fn main() {
    let (n, x): (usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let mut ab: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        ab.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        });
    }

    let mut cum_sum = vec![ab[0].0 + ab[0].1];
    for i in 0..(n - 1) {
        cum_sum.push(cum_sum[i] + ab[i + 1].0 + ab[i + 1].1);
    }

    let mut cum_min_b = vec![ab[0].1];
    for i in 0..(n - 1) {
        cum_min_b.push(cum_min_b[i].min(ab[i + 1].1));
    }

    let mut ans = 18446744073709551615;

    // i番目のステージまで使う
    for i in 0..(n.min(x)) {
        let cost = cum_sum[i] + (x - (i + 1)) * cum_min_b[i];
        ans = ans.min(cost);
    }

    println!("{}", ans);
}
