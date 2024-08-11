use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        nk: [(usize, usize); t],
    }

    let solve = |n: usize, k: usize| {
        if k == 1 {
            return (0..n).map(|_| "1").join(" ");
        }

        let raised = 1_usize << k - 1;

        (0..n)
            .map(|i| {
                if i >= raised {
                    return 1;
                }

                let mut value = format!("{:b}", i).chars().rev().collect::<String>();
                value.push_str(&"0".repeat(k - 1 - value.len()));
                value.insert(0, '1');

                usize::from_str_radix(&value, 2).unwrap()
            })
            .join(" ")
    };

    let ans = nk.iter().map(|&(n, k)| solve(n, k)).join("\n");
    println!("{}", ans);
}
