use proconio::input;

fn main() {
    let n = 1_usize << 20;
    input! {
        q: usize,
        tx: [(usize, usize); q],
    }

    let mut aa = vec![-1_i64; n];
    for (t, x) in tx {
        if t == 1 {
            let mut h = x;
            while aa[h % n] != -1 {
                h += 1;
            }
            aa[h % n] = x as i64;
        } else {
            println!("{}", aa[x % n]);
        }
    }
}

struct UnionFind {
    parents: Vec<i64>,
    group_num: i64,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: i64) -> Self {
        UnionFind {
            parents: vec![-1; n as usize],
            group_num: n,
        }
    }

    fn root(&mut self, x: i64) -> i64 {
        if self.parents[x as usize] < 0 {
            return x;
        }
        self.parents[x as usize] = self.root(self.parents[x as usize]);
        self.parents[x as usize]
    }

    fn is_same(&mut self, x: i64, y: i64) -> bool {
        self.root(x) == self.root(y)
    }

    fn merge(&mut self, x: i64, y: i64) -> bool {
        let mut root_x = self.root(x);
        let mut root_y = self.root(y);
        if root_x == root_y {
            return false;
        }
        if self.parents[root_x as usize] > self.parents[root_y as usize] {
            std::mem::swap(&mut root_x, &mut root_y);
        }
        self.parents[root_x as usize] += self.parents[root_y as usize];
        self.parents[root_y as usize] = root_x;
        self.group_num -= 1;
        true
    }

    fn size(&mut self, x: i64) -> i64 {
        let root_x = self.root(x);
        -self.parents[root_x as usize]
    }

    fn add(&mut self) {
        self.parents.push(-1);
        self.group_num += 1;
    }

    fn get_group_num(&self) -> i64 {
        self.group_num
    }
}
