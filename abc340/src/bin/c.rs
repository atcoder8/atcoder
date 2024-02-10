use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = rec(n, &mut HashMap::new());
    println!("{}", ans);
}

fn rec(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n <= 1 {
        return 0;
    }

    if let Some(&cost) = memo.get(&n) {
        return cost;
    }

    let cost = n + rec(n / 2, memo) + rec((n + 1) / 2, memo);
    memo.insert(n, cost);

    cost
}
