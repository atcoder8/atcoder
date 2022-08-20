use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
        (h, w, k, x1, y1, x2, y2): (i64, i64, i64, i64, i64, i64, i64),
    }

    let g00: i64 = if x1 == x2 && y1 == y2 { 1 } else { 0 };
    let g01: i64 = if x1 == x2 && y1 != y2 { 1 } else { 0 };
    let g10: i64 = if x1 != x2 && y1 == y2 { 1 } else { 0 };
    let g11: i64 = if x1 != x2 && y1 != y2 { 1 } else { 0 };
    let gg = vec![vec![g00, g01, g10, g11]].transpose();
    let aa = vec![
        vec![0, 1, 1, 0],
        vec![w - 1, w - 2, 0, 1],
        vec![h - 1, 0, h - 2, 1],
        vec![0, h - 1, w - 1, h + w - 4],
    ]
    .pow_mod(k, MOD).mul_mod(&gg, MOD);

    println!("{}", aa[0][0]);
}

trait MatOps {
    fn get_shape(&self) -> (usize, usize);
    fn transpose(&self) -> Vec<Vec<i64>>;
    fn add(&self, rhs: &Vec<Vec<i64>>) -> Vec<Vec<i64>>;
    fn sub(&self, rhs: &Vec<Vec<i64>>) -> Vec<Vec<i64>>;
    fn mul(&self, rhs: &Vec<Vec<i64>>) -> Vec<Vec<i64>>;
    fn mul_mod(&self, rhs: &Vec<Vec<i64>>, m: i64) -> Vec<Vec<i64>>;
    fn pow(&self, n: i64) -> Vec<Vec<i64>>;
    fn pow_mod(&self, n: i64, m: i64) -> Vec<Vec<i64>>;
}

impl MatOps for Vec<Vec<i64>> {
    fn get_shape(&self) -> (usize, usize) {
        let rows = self.len();
        assert!(rows != 0);
        let cols = self[0].len();
        assert!(cols != 0);
        for i in 1..rows {
            assert!(self[i].len() == cols);
        }
        return (rows, cols);
    }

    fn transpose(&self) -> Vec<Vec<i64>> {
        let (rows, cols) = self.get_shape();
        let mut ret = vec![vec![0; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                ret[j][i] = self[i][j];
            }
        }
        ret
    }

    fn add(&self, rhs: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        let (rows1, cols1) = self.get_shape();
        let (rows2, cols2) = rhs.get_shape();
        assert_eq!(rows1, rows2);
        assert_eq!(cols1, cols2);
        let mut ret = self.clone();
        for i in 0..rows1 {
            for j in 0..cols1 {
                ret[i][j] += rhs[i][j];
            }
        }
        ret
    }

    fn sub(&self, rhs: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        let (rows1, cols1) = self.get_shape();
        let (rows2, cols2) = rhs.get_shape();
        assert_eq!(rows1, rows2);
        assert_eq!(cols1, cols2);
        let mut ret = self.clone();
        for i in 0..rows1 {
            for j in 0..cols1 {
                ret[i][j] -= rhs[i][j];
            }
        }
        ret
    }

    fn mul(&self, rhs: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        let (rows1, cols1) = self.get_shape();
        let (rows2, cols2) = rhs.get_shape();
        assert_eq!(cols1, rows2);
        let mut ret = vec![vec![0; cols2]; rows1];
        for i in 0..rows1 {
            for j in 0..cols2 {
                for k in 0..cols1 {
                    ret[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        ret
    }

    fn mul_mod(&self, rhs: &Vec<Vec<i64>>, m: i64) -> Vec<Vec<i64>> {
        assert!(m >= 1);
        let (rows1, cols1) = self.get_shape();
        let (rows2, cols2) = rhs.get_shape();
        assert_eq!(cols1, rows2);
        let mut ret = vec![vec![0; cols2]; rows1];
        for i in 0..rows1 {
            for j in 0..cols2 {
                for k in 0..cols1 {
                    ret[i][j] = (ret[i][j] + (self[i][k] % m) * (rhs[k][j] % m) % m) % m;
                }
            }
        }
        ret
    }

    fn pow(&self, mut n: i64) -> Vec<Vec<i64>> {
        assert!(n >= 0);
        let (rows, cols) = self.get_shape();
        assert_eq!(rows, cols);
        let mut ret = vec![vec![0; rows]; rows];
        for i in 0..rows {
            ret[i][i] = 1;
        }
        let mut temp = self.clone();
        while n != 0 {
            if n & 1 == 1 {
                ret = ret.mul(&temp);
            }
            temp = temp.mul(&temp);
            n >>= 1;
        }
        ret
    }

    fn pow_mod(&self, mut n: i64, m: i64) -> Vec<Vec<i64>> {
        assert!(n >= 0 && m >= 1);
        let (rows, cols) = self.get_shape();
        assert_eq!(rows, cols);
        let mut ret = vec![vec![0; rows]; rows];
        for i in 0..rows {
            ret[i][i] = 1 % m;
        }
        let mut temp = self.clone();
        while n != 0 {
            if n & 1 == 1 {
                ret = ret.mul_mod(&temp, m);
            }
            temp = temp.mul_mod(&temp, m);
            n >>= 1;
        }
        ret
    }
}
