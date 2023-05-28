use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        if let Some(ans) = solve() {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}

fn solve() -> Option<String> {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
        s: Chars,
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut colors = vec!['\0'; n];
    let mut adjacent_black_num = vec![0; n];
    let mut adjacent_white_num = vec![0; n];

    let mut success = rec(
        &graph,
        &s,
        &mut colors,
        &mut adjacent_black_num,
        &mut adjacent_white_num,
        None,
        0,
    );

    success &= (s[0] == 'B' && adjacent_black_num[0] > adjacent_white_num[0])
        || (s[0] == 'W' && adjacent_black_num[0] < adjacent_white_num[0]);

    if success {
        Some(colors.iter().collect())
    } else {
        None
    }
}

fn rec(
    graph: &Vec<Vec<usize>>,
    s: &Vec<char>,
    colors: &mut Vec<char>,
    adjacent_black_num: &mut Vec<usize>,
    adjacent_white_num: &mut Vec<usize>,
    par: Option<usize>,
    cur: usize,
) -> bool {
    let mut black_flag = true;
    let mut white_flag = true;

    for &next in &graph[cur] {
        if Some(next) == par {
            continue;
        }

        let success = rec(
            graph,
            s,
            colors,
            adjacent_black_num,
            adjacent_white_num,
            Some(cur),
            next,
        );

        if !success {
            return false;
        }

        if s[next] == 'B' {
            if adjacent_black_num[next] < adjacent_white_num[next] {
                return false;
            }

            if adjacent_black_num[next] == adjacent_white_num[next] {
                white_flag = false;
            }
        } else {
            if adjacent_black_num[next] > adjacent_white_num[next] {
                return false;
            }

            if adjacent_black_num[next] == adjacent_white_num[next] {
                black_flag = false;
            }
        }
    }

    colors[cur] = if black_flag && white_flag {
        if let Some(par) = par {
            s[par]
        } else {
            'B'
        }
    } else if black_flag {
        'B'
    } else if white_flag {
        'W'
    } else {
        return false;
    };

    if colors[cur] == 'B' {
        for &next in &graph[cur] {
            adjacent_black_num[next] += 1;
        }
    } else {
        for &next in &graph[cur] {
            adjacent_white_num[next] += 1;
        }
    }

    true
}
