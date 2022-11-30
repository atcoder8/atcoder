use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let ans = ss.iter().enumerate().any(|(i, s)| {
        !rec(
            &mut vec![None; 1 << ss.len()],
            &ss,
            *s.last().unwrap(),
            1 << i,
        )
    });
    println!("{}", if ans { "First" } else { "Second" });
}

pub fn rec(memo: &mut Vec<Option<bool>>, ss: &Vec<Vec<char>>, prev_c: char, used: usize) -> bool {
    if let Some(ret) = memo[used] {
        ret
    } else {
        let ret = (0..ss.len())
            .filter(|&i| (used >> i) & 1 == 0 && *ss[i].first().unwrap() == prev_c)
            .any(|i| !rec(memo, ss, *ss[i].last().unwrap(), used | (1 << i)));
        memo[used] = Some(ret);

        ret
    }
}
