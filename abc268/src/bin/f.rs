// unfinished

fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let mut ccc: Vec<Vec<char>> = vec![];
    for _ in 0..n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        ccc.push(line.trim().chars().collect());
    }

    ccc.sort_by_cached_key(|cc| calc_score(cc));

    let connected_ccc: Vec<&char> = ccc.iter().flatten().collect();

    let mut ans = 0;
    let mut x_cnt = 0;

    for &c in connected_ccc {
        if c == 'X' {
            x_cnt += 1;
        } else {
            ans += c.to_digit(10).unwrap() as usize * x_cnt;
        }
    }

    println!("{}", ans);
}

fn calc_score(cc: &[char]) -> usize {
    let mut score = 0;
    let mut x_cnt = 1;

    for &c in cc.iter() {
        if c == 'X' {
            x_cnt += 1;
        } else {
            score += c.to_digit(10).unwrap() as usize * x_cnt;
        }
    }

    score
}
