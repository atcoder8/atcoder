use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    for &a in &aa {
        *map.entry(a).or_insert(0) += 1;
    }

    let ans: usize = map.values().map(|&v| v / 2).sum();
    println!("{}", ans);
}
