// unfinished

use std::ops::Add;

use proconio::input;

const MAX_EXP: u32 = 25;
const RAISED: usize = 2_usize.pow(MAX_EXP);

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut tree = vec![Node::default(); 2_usize.pow(MAX_EXP + 1)];
    for &a in &aa {
        tree[RAISED + a] = Node::new(1, a);
    }
    for i in (1..RAISED).rev() {
        tree[i] = tree[2 * i] + tree[2 * i + 1];
    }

    let mut ans = 0_usize;
    for &a in &aa {
        // 末尾に0がexp個続く
        let trailing = !a + 1;
        for exp in 0..MAX_EXP {
            let mask = 2_usize.pow(exp) - 1;
            let complementary = trailing & mask ^ 1;
            let Node { count, sum } = tree[complementary];
            ans += (a * count + sum) >> exp;
        }
    }
    println!("{}", ans);
}

#[derive(Debug, Clone, Copy, Default)]
struct Node {
    count: usize,
    sum: usize,
}

impl Add<Node> for Node {
    type Output = Node;

    fn add(self, rhs: Node) -> Self::Output {
        Self::Output {
            count: self.count + rhs.count,
            sum: self.sum + rhs.sum,
        }
    }
}

impl Node {
    fn new(count: usize, sum: usize) -> Self {
        Self { count, sum }
    }
}
