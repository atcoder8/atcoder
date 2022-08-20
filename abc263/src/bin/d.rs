fn main() {
    let (n, l, r) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        )
    };
    let aa = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    };

    let mut acc = vec![0];
    for (i, &a) in aa.iter().enumerate() {
        acc.push(acc[i] + a);
    }

    let left_diffs: Vec<i64> = aa.iter().map(|&a| l - a).collect();
    let mut acc_left_diffs = vec![0];
    for (i, &x) in left_diffs.iter().enumerate() {
        acc_left_diffs.push(acc_left_diffs[i] + x);
    }
    let mut min_left = vec![0; n + 1];
    for i in 0..n {
        min_left[i + 1] = acc_left_diffs[i + 1].min(min_left[i]);
    }

    let right_diffs: Vec<i64> = aa.iter().rev().map(|&a| r - a).collect();
    let mut acc_right_diffs = vec![0];
    for (i, &x) in right_diffs.iter().enumerate() {
        acc_right_diffs.push(acc_right_diffs[i] + x);
    }
    let mut min_right = vec![0; n + 1];
    for i in 0..n {
        min_right[i + 1] = acc_right_diffs[i + 1].min(min_right[i]);
    }

    let total_aa: i64 = aa.iter().sum();
    let mut ans = total_aa;

    for i in 0..(n + 1) {
        ans = ans.min(total_aa + min_left[i] + min_right[n - i]);
    }

    println!("{}", ans);
}
