use itertools::{join, Itertools};
use permutohedron::LexicalPermutation;

fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };

    let grid = if n == 3 {
        brute_force_search(n)
    } else if n % 2 == 0 {
        even_solve(n)
    } else {
        odd_solve(n)
    };

    debug_assert_eq!(
        {
            let mut sorted_flatten_grid = grid.clone().into_iter().flatten().collect_vec();
            sorted_flatten_grid.sort_unstable();
            sorted_flatten_grid
        },
        (1..=n.pow(2)).collect_vec()
    );

    debug_assert!(check_match_condition(&grid));

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

fn brute_force_search(n: usize) -> Vec<Vec<usize>> {
    let sq_n = n * n;

    let mut perm = (1..=sq_n).collect_vec();

    loop {
        let mut grid = vec![vec![0; n]; n];

        for (cell_idx, &val) in perm.iter().enumerate() {
            grid[cell_idx / n][cell_idx % n] = val;
        }

        if check_match_condition(&grid) {
            break grid;
        }

        perm.next_permutation();
    }
}

fn even_solve(n: usize) -> Vec<Vec<usize>> {
    let floor_half_n = n / 2;

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

    odd_groups.iter_mut().for_each(|group| group.reverse());

    let mut odd_rem = 0;

    let mut use_pair = |odd_coord: (usize, usize), even_coord: (usize, usize)| {
        let even_rem = (3 - odd_rem) % 3;

        grid[odd_coord.0][odd_coord.1] = odd_groups[odd_rem].pop().unwrap();
        grid[even_coord.0][even_coord.1] = even_groups[even_rem].pop().unwrap();

        if odd_groups[odd_rem].is_empty() || even_groups[even_rem].is_empty() {
            odd_rem += 1;
        }
    };

    for i in 0..n {
        use_pair((floor_half_n - 1, i), (floor_half_n, i));
    }

    let mut odds = odd_groups.into_iter().flatten().collect_vec();
    let mut evens = even_groups.into_iter().flatten().collect_vec();

    for i in 0..(floor_half_n - 1) {
        for j in 0..n {
            grid[i][j] = odds.pop().unwrap();
        }
    }

    for i in (floor_half_n + 1)..n {
        for j in 0..n {
            grid[i][j] = evens.pop().unwrap();
        }
    }

    grid
}

fn odd_solve(n: usize) -> Vec<Vec<usize>> {
    let floor_half_n = n / 2;

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

    odd_groups.iter_mut().for_each(|group| group.reverse());

    let mut odd_rem = 0;

    let mut use_pair = |odd_coord: (usize, usize), even_coord: (usize, usize)| {
        let even_rem = (3 - odd_rem) % 3;

        grid[odd_coord.0][odd_coord.1] = odd_groups[odd_rem].pop().unwrap();
        grid[even_coord.0][even_coord.1] = even_groups[even_rem].pop().unwrap();

        if odd_groups[odd_rem].is_empty() || even_groups[even_rem].is_empty() {
            odd_rem += 1;
        }
    };

    use_pair(
        (floor_half_n, floor_half_n),
        (floor_half_n + 1, floor_half_n),
    );

    use_pair(
        (floor_half_n - 1, floor_half_n + 1),
        (floor_half_n, floor_half_n + 1),
    );

    for i in 0..floor_half_n {
        use_pair((floor_half_n, i), (floor_half_n + 1, i));
    }

    for i in (floor_half_n + 2)..n {
        use_pair((floor_half_n - 1, i), (floor_half_n, i));
    }

    let mut odds = odd_groups.into_iter().flatten().collect_vec();
    let mut evens = even_groups.into_iter().flatten().collect_vec();

    for i in 0..floor_half_n {
        for j in 0..n {
            if grid[i][j] == 0 {
                grid[i][j] = odds.pop().unwrap();
            }
        }
    }

    for i in (floor_half_n + 1)..n {
        for j in 0..n {
            if grid[i][j] == 0 {
                grid[i][j] = evens.pop().unwrap();
            }
        }
    }

    grid
}
