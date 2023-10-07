use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        sc: [(u128, u128); n],
    }

    let mut ans = 0;
    let mut map: BTreeMap<u128, u128> = sc.into_iter().collect();
    while let Some((s, c)) = map.pop_first() {
        ans += c % 2;

        if c / 2 != 0 {
            *map.entry(2 * s).or_default() += c / 2;
        }
    }
    println!("{}", ans);
}
