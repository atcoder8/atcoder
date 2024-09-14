use itertools::Itertools;
use ndarray::Array2;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mg: usize,
        edges1: [(Usize1, Usize1); mg],
        mh: usize,
        edges2: [(Usize1, Usize1); mh],
    }

    let mut cost_mat = Array2::from_elem((n, n), 0_usize);
    for u in 0..n {
        for v in u + 1..n {
            input! {
                a: usize,
            }

            cost_mat[(u, v)] = a;
            cost_mat[(v, u)] = a;
        }
    }

    let create_matrix = |edges: &[(usize, usize)]| {
        let mut matrix = Array2::from_elem((n, n), false);
        for &(u, v) in edges {
            matrix[(u, v)] = true;
            matrix[(v, u)] = true;
        }

        matrix
    };

    let matrix1 = create_matrix(&edges1);
    let matrix2 = create_matrix(&edges2);

    let calc_cost = |perm: &[usize]| {
        let mut sum_cost = 0;
        for u in 0..n {
            for v in u + 1..n {
                if matrix1[(perm[u], perm[v])] != matrix2[(u, v)] {
                    sum_cost += cost_mat[(u, v)];
                }
            }
        }

        sum_cost
    };

    let ans = (0..n)
        .permutations(n)
        .map(|perm| calc_cost(&perm))
        .min()
        .unwrap();
    println!("{}", ans);
}
