// unfinished

use itertools::{join, Itertools};

fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };

    let grid = if n % 2 == 0 { even_solve(n) } else { todo!() };

    assert!(check_match_condition(&grid));

    for line in grid {
        println!("{}", join(line, " "));
    }
}

fn primality_test(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    for i in (2..).take_while(|x| x * x <= n) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn check_match_condition(grid: &Vec<Vec<usize>>) -> bool {
    let n = grid.len();

    !(0..n.pow(2)).any(|cell_idx| {
        let (i, j) = (cell_idx / n, cell_idx % n);
        let cur_val = grid[i][j];

        (i > 0 && primality_test(cur_val + grid[i - 1][j]))
            || (i < n - 1 && primality_test(cur_val + grid[i + 1][j]))
            || (j > 0 && primality_test(cur_val + grid[i][j - 1]))
            || (j < n - 1 && primality_test(cur_val + grid[i][j + 1]))
    })
}

// fn tle(n: usize) {
//     let sq_n = n * n;

//     let mut perm = (1..=sq_n).collect_vec();

//     while {
//         let mut grid = vec![vec![0; n]; n];

//         for (cell_idx, &val) in perm.iter().enumerate() {
//             grid[cell_idx / n][cell_idx % n] = val;
//         }

//         if check_match_condition(&grid) {
//             for line in grid {
//                 println!("{}", join(line, ""));
//             }
//             io::stdout().flush().unwrap();

//             return;
//         }

//         perm.next_permutation()
//     } {}
// }

fn even_solve(n: usize) -> Vec<Vec<usize>> {
    let half_n = n / 2;

    let mut grid = vec![vec![0; n]; n];

    let mut odd_groups = vec![vec![]; 3];
    let mut even_groups = vec![vec![]; 3];

    for i in 1..=n.pow(2) {
        if i % 2 == 1 {
            odd_groups[i % 3].push(i);
        } else {
            even_groups[i % 3].push(i);
        }
    }

    even_groups.iter_mut().for_each(|group| group.reverse());

    let mut odd_rem = 0;
    let mut cnt = 0;

    for i in 0..n {
        let mut even_rem = (3 - odd_rem) % 3;

        if cnt >= odd_groups[odd_rem].len() || cnt >= even_groups[even_rem].len() {
            odd_rem += 1;
            even_rem = (3 - odd_rem) % 3;
            while odd_groups[odd_rem].is_empty() || even_groups[even_rem].is_empty() {
                odd_rem += 1;
                even_rem = (3 - odd_rem) % 3;
            }
            cnt = 0;
        }

        grid[i][half_n - 1] = odd_groups[odd_rem][cnt];
        grid[i][half_n] = even_groups[even_rem][cnt];

        cnt += 1;
    }

    let mut odds = odd_groups.into_iter().flatten().collect_vec();
    let mut evens = even_groups.into_iter().flatten().collect_vec();

    for i in 0..n {
        for j in 0..(half_n - 1) {
            grid[i][j] = odds.pop().unwrap();
        }

        for j in (half_n + 1)..n {
            grid[i][j] = evens.pop().unwrap();
        }
    }

    grid
}
