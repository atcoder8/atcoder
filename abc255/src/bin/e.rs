use im_rc::HashMap;

fn main() {
    let (n, _m): (usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let ss: Vec<i64> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let xx: Vec<i64> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    let mut zero_aa = vec![0];
    for i in 0..(n - 1) {
        zero_aa.push(ss[i] - zero_aa[i]);
    }

    let mut plus = HashMap::new();
    let mut minus = HashMap::new();
    for i in 0..n {
        if i % 2 == 0 {
            let count = plus.entry(&zero_aa[i]).or_insert(0);
            *count += 1;
        } else {
            let count = minus.entry(&zero_aa[i]).or_insert(0);
            *count += 1;
        }
    }

    let mut ans = 0;
    for x in xx.iter() {
        for j in 0..n {
            let diff = if j % 2 == 0 {
                zero_aa[j] - *x
            } else {
                *x - zero_aa[j]
            };

            let mut cnt = 0;
            for x2 in xx.iter() {
                if let Some(match_cnt) = plus.get(&(*x2 + diff)) {
                    cnt += match_cnt;
                }
                if let Some(match_cnt) = minus.get(&(*x2 - diff)) {
                    cnt += match_cnt;
                }
            }

            ans = std::cmp::max(ans, cnt);
        }
    }

    println!("{}", ans);
}
