use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        pp: [Usize1; n],
    }

    let mut cycles = vec![];
    let mut visited = vec![false; n];
    for start in 0..n {
        if visited[start] {
            continue;
        }

        visited[start] = true;

        let mut cycle = vec![start];
        let mut cur = pp[start];
        while cur != start {
            cycle.push(cur);
            cur = pp[cur];
            visited[cur] = true;
        }

        cycles.push(cycle);
    }

    let mut destinations = vec![n; n];
    for cycle in &cycles {
        let rem = pow_mod(2, k, cycle.len());
        for (i, &from) in enumerate(cycle) {
            let to = cycle[(i + rem) % cycle.len()];
            destinations[from] = to;
        }
    }

    println!("{}", destinations.iter().map(|dest| dest + 1).join(" "));
}

/// Calculate the remainder of `exp` power of `base` divided by `m`.
pub fn pow_mod(base: usize, exp: usize, m: usize) -> usize {
    let mut ret = 1 % m;
    let mut mul = base % m;
    let mut t = exp;

    while t != 0 {
        if t & 1 == 1 {
            ret = ret * mul % m;
        }

        mul = mul * mul % m;
        t >>= 1;
    }

    ret
}

pub fn floor_log2(n: usize) -> u32 {
    (0_u32..).find(|&i| n >> i <= 1).unwrap()
}
