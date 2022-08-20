fn main() {
    let (n, m, k) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let mut aa = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };
    aa.sort();

    let mut bb = aa.clone();
    let mut sum_diffs = vec![0; n];
    let mut ans = 0;

    for target in (0..32).rev() {
        let mask = (1 << target) - 1;
        let mut added_bb = bb.clone();
        let mut diffs = vec![0; n];

        for i in 0..n {
            if (bb[i] >> target) & 1 == 0 {
                diffs[i] = (1 << target) - (bb[i] & mask);
                added_bb[i] += diffs[i];
            }
        }

        let next_sum_diffs: Vec<usize> = sum_diffs
            .iter()
            .zip(diffs.iter())
            .map(|(x, y)| *x + *y)
            .collect();

        let mut sorted_next_sum_diffs = next_sum_diffs.clone();
        sorted_next_sum_diffs.sort_unstable();
        let cost: usize = sorted_next_sum_diffs[..k].iter().sum();

        if cost <= m {
            ans += 1_usize << target;
            bb = added_bb;
            sum_diffs = next_sum_diffs;
        }
    }

    println!("{}", ans);
}
