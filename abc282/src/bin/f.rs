use std::io::{self, Write};

use superslice::Ext;

fn main() {
    // Phase 1
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };

    let mut sections: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    let mut section_cnt = 0;

    for left in 0..n {
        let mut len = 1;

        while left + len <= n {
            let right = left + len;

            sections[left].push((right, section_cnt));

            section_cnt += 1;
            len *= 2;
        }
    }

    println!("{}", section_cnt);
    for left in 0..n {
        for &(right, _) in &sections[left] {
            println!("{} {}", left + 1, right);
        }
    }
    io::stdout().flush().unwrap();

    // Phase 2
    let q = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };

    for _ in 0..q {
        let (l, r) = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        };

        let a_idx = sections[l].upper_bound_by_key(&r, |x| x.0) - 1;
        let a = sections[l][a_idx].1 + 1;

        let floor_log2_len = floor_log2(r - l);
        let b = sections[r - (1 << floor_log2_len)][floor_log2_len].1 + 1;

        println!("{} {}", a, b);
        io::stdout().flush().unwrap();
    }
}

pub fn floor_log2(x: usize) -> usize {
    assert_ne!(x, 0);

    let mut exp = 0;
    let mut val = 1;

    loop {
        if val > x / 2 {
            return exp;
        }

        exp += 1;
        val *= 2;
    }
}
