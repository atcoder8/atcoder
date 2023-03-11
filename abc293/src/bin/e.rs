use proconio::input;

fn main() {
    input! {
        (a, x, m): (usize, usize, usize),
    }

    let mut mat = vec![vec![1, 0], vec![0, 1]];
    let mut mul = vec![vec![a, 1], vec![0, 1]];

    let mut t = x;
    while t != 0 {
        if t % 2 == 1 {
            let mut next_mat = vec![vec![0; 2]; 2];
            for i in 0..2 {
                for j in 0..2 {
                    next_mat[i][j] = (0..2).map(|k| mat[i][k] * mul[k][j]).sum::<usize>() % m;
                }
            }
            mat = next_mat;
        }

        let mut next_mul = vec![vec![0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                next_mul[i][j] = (0..2).map(|k| mul[i][k] * mul[k][j]).sum::<usize>() % m;
            }
        }
        mul = next_mul;

        t /= 2;
    }

    let ans = mat[0][1];
    println!("{}", ans);
}
