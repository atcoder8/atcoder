use im_rc::HashMap;
use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "First" } else { "Second" });
}

fn solve() -> bool {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let mut memo: HashMap<usize, bool> = HashMap::new();

    (0..ss.len()).any(|i| !rec(&mut memo, &ss, *ss[i].last().unwrap(), 1 << i))
}

pub fn rec(
    memo: &mut HashMap<usize, bool>,
    ss: &Vec<Vec<char>>,
    prev_c: char,
    used: usize,
) -> bool {
    if let Some(&ret) = memo.get(&used) {
        ret
    } else {
        let ret = (0..ss.len())
            .filter(|&i| (used >> i) & 1 == 0 && *ss[i].first().unwrap() == prev_c)
            .any(|i| !rec(memo, ss, *ss[i].last().unwrap(), used | (1 << i)));
        memo.insert(used, ret);
        ret
    }
}
