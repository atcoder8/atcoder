fn main() {
    let aa: Vec<i32> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    let hh: Vec<i32> = (0..3).map(|i| aa[i]).collect();
    let ww: Vec<i32> = (3..6).map(|i| aa[i]).collect();

    let mut ans = 0;
    for ul in 1..(hh[0] - 1) {
        for ur in 1..(hh[0] - ul) {
            for dl in 1..(hh[1] - 1) {
                for dr in 1..(hh[1] - dl) {
                    let mut mat = vec![vec![ul, ur, 0], vec![dl, dr, 0], vec![0, 0, 0]];
                    mat[0][2] = hh[0] - (mat[0][0] + mat[0][1]);
                    mat[1][2] = hh[1] - (mat[1][0] + mat[1][1]);
                    mat[2][0] = ww[0] - (mat[0][0] + mat[1][0]);
                    mat[2][1] = ww[1] - (mat[0][1] + mat[1][1]);
                    mat[2][2] = hh[2] - (mat[2][0] + mat[2][1]);

                    let mut flag = true;
                    for i in 0..3 {
                        for j in 0..3 {
                            if mat[i][j] <= 0 {
                                flag = false;
                            }
                        }
                    }
                    if !flag {
                        continue;
                    }

                    if (0..3).all(|i| mat[i][0] + mat[i][1] + mat[i][2] == hh[i]) && (0..3).all(|i| mat[0][i] + mat[1][i] + mat[2][i] == ww[i]) {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
