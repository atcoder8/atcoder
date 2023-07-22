use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, d): (usize, usize),
        ss: [Chars; n],
    }

    let ans = (0..d)
        .map(|start| {
            (start..d)
                .take_while(|&day| ss.iter().all(|s| s[day] == 'o'))
                .count()
        })
        .max()
        .unwrap();

    println!("{}", ans);
}
