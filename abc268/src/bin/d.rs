use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    if let Some(ans) = solve() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<String> {
    let (n, m) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let mut sss: Vec<Vec<char>> = vec![];
    for _ in 0..n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        sss.push(line.trim().chars().collect());
    }
    let mut ttt: Vec<Vec<char>> = vec![];
    for _ in 0..m {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        ttt.push(line.trim().chars().collect());
    }

    sss.sort_unstable();

    let mut ttt_set = HashSet::new();
    for tt in ttt {
        ttt_set.insert(tt);
    }

    if n == 1 {
        return if sss[0].len() >= 3 && !ttt_set.contains(&sss[0]) {
            Some(sss[0].iter().collect())
        } else {
            None
        };
    }

    let min_len = sss.iter().map(|ss| ss.len()).sum::<usize>() + n - 1;
    if min_len > 16 {
        return None;
    }

    for sss_perm in sss.iter().permutations(n) {
        let mut que = VecDeque::from(vec![(vec![0; n - 1], 0)]);

        while let Some((add_counts, add_idx)) = que.pop_front() {
            if add_idx == n - 1 {
                let mut chars = sss_perm[0].clone();
                for i in 1..n {
                    chars.resize(chars.len() + add_counts[i - 1] + 1, '_');
                    chars.extend(sss_perm[i].iter());
                }

                if !ttt_set.contains(&chars) {
                    return Some(chars.iter().collect());
                }

                continue;
            }

            let rem = 16 - min_len - add_counts.iter().sum::<usize>();
            for add in 0..=rem {
                let mut next_add_counts = add_counts.clone();
                next_add_counts[add_idx] += add;
                que.push_back((next_add_counts, add_idx + 1));
            }
        }
    }

    None
}
