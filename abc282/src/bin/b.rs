use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, _m): (usize, usize),
        ss: [Chars; n],
    }

    let mut ans = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            ans += ss[i]
                .iter()
                .zip(ss[j].iter())
                .all(|(&c1, &c2)| c1 == 'o' || c2 == 'o') as usize;
        }
    }

    println!("{}", ans);
}
