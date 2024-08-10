use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let create_string = |row: usize| {
        let mut chars = (0..n)
            .rev()
            .map(|col| ss[col].get(row).cloned().unwrap_or('*'))
            .collect::<Vec<_>>();
        while chars.last() == Some(&'*') {
            chars.pop();
        }

        chars.iter().collect::<String>()
    };

    let max_len = ss.iter().map(|s| s.len()).max().unwrap();
    let ans = (0..max_len).map(create_string).join("\n");
    println!("{}", ans);
}
