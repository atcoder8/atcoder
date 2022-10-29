use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut memo: HashMap<usize, usize> = HashMap::new();

    println!("{}", f(n, &mut memo));
}

fn f(x: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if x == 0 {
        return 1;
    }

    let val1 = if let Some(&val) = memo.get(&(x / 2)) {
        val
    } else {
        let val = f(x / 2, memo);
        memo.insert(x / 2, val);

        val
    };

    let val2 = if let Some(&val) = memo.get(&(x / 3)) {
        val
    } else {
        let val = f(x / 3, memo);
        memo.insert(x / 3, val);

        val
    };

    memo.insert(x, val1 + val2);

    val1 + val2
}
