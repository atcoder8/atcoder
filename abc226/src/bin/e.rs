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

fn pow_mod(mut x: i64, mut n: i64, m: i64) -> i64 {
    assert!(n >= 0 && m >= 1);
    x %= m;
    let mut ret = 1 % m;
    while n != 0 {
        if n & 1 == 1 {
            ret = ret * x % m;
        }
        x = x * x % m;
        n >>= 1;
    }
    ret
}

fn main() {
    proconio::input! {
        n: i64,
        m: i64,
        mut uv: [(i64, i64); m],
    }

    uv.iter_mut().for_each(|x| { x.0 -= 1; x.1 -= 1 });

    if n != m {
        println!("0");
        return;
    }
    let mut uf = UnionFind::new(n);
    for (u, v) in uv.iter() {
        uf.merge(*u, *v);
    }
    let mut counts = vec![0; n as usize];
    for (u, _) in uv.iter() {
        counts[uf.root(*u) as usize] += 1;
    }
    for i in 0..n {
        if i == uf.root(i) && counts[i as usize] != uf.size(i) {
            println!("0");
            return;
        }
    }
    println!("{}", pow_mod(2, uf.get_group_num(), 998244353));
}
