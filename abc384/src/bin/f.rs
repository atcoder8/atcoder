use std::ops::{Add, AddAssign, Sub};

use proconio::input;

const MAX_EXP: u32 = 24;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    // 各expについて2^expを法として合同である要素をグループ化する
    let mut init_nodes = vec![Node::default(); 2_usize.pow(MAX_EXP)];
    for &a in &aa {
        init_nodes[a] += Node::new(a);
    }
    let mut node_tree: Vec<Vec<Node>> = vec![vec![]; MAX_EXP as usize + 1];
    node_tree[MAX_EXP as usize] = init_nodes;
    for exp in (0..MAX_EXP).rev() {
        let from_nodes = &node_tree[exp as usize + 1];
        let mut nodes = vec![Node::default(); 2_usize.pow(exp)];
        for i in 0..2_usize.pow(exp) {
            nodes[i] = from_nodes[i] + from_nodes[i + 2_usize.pow(exp)];
        }
        node_tree[exp as usize] = nodes;
    }

    let mut ans = 0_usize;
    for &a in &aa {
        let complementary = a.wrapping_neg();
        for exp in 0..MAX_EXP {
            let raised = 2_usize.pow(exp);
            let Node { count, sum } =
                node_tree[exp as usize + 1][complementary % (2 * raised) ^ raised];
            ans += (a * count + sum) >> exp;
        }
        let Node { count, sum } = node_tree[MAX_EXP as usize][complementary % 2_usize.pow(MAX_EXP)];
        ans += (a * count + sum) >> MAX_EXP;
    }
    ans += aa.iter().map(|&a| a >> a.trailing_zeros()).sum::<usize>();
    ans /= 2;
    println!("{}", ans);
}

#[derive(Debug, Clone, Copy, Default)]
struct Node {
    count: usize,
    sum: usize,
}

impl Add<Self> for Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            count: self.count + rhs.count,
            sum: self.sum + rhs.sum,
        }
    }
}

impl Sub<Self> for Node {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            count: self.count - rhs.count,
            sum: self.sum - rhs.sum,
        }
    }
}

impl AddAssign<Self> for Node {
    fn add_assign(&mut self, rhs: Self) {
        self.count += rhs.count;
        self.sum += rhs.sum;
    }
}

impl Node {
    fn new(a: usize) -> Self {
        Self { count: 1, sum: a }
    }
}
