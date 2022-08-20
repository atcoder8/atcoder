fn main() {
    let (n, m) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let mut uv = Vec::new();
    for _ in 0..m {
        uv.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
            )
        });
    }

    let mut mat = vec![vec![false; n]; n];
    for &(u, v) in uv.iter() {
        mat[u][v] = true;
        mat[v][u] = true;
    }

    let mut ans = 0;
    for i in 0..(n - 2) {
        for j in (i + 1)..(n - 1) {
            for k in (j + 1)..n {
                if mat[i][j] && mat[j][k] && mat[k][i] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
