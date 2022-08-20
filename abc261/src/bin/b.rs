fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let mut aaa: Vec<Vec<char>> = vec![];
    for _ in 0..n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        aaa.push(line.trim().chars().collect());
    }

    if solve(&aaa) {
        println!("correct");
    } else {
        println!("incorrect");
    }
}

fn solve(aaa: &Vec<Vec<char>>) -> bool {
    let n = aaa.len();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let (a, b) = (aaa[i][j], aaa[j][i]);
            if (a == 'W' && b != 'L') || (a == 'L' && b != 'W') || (a == 'D' && b != 'D') {
                return false;
            }
        }
    }

    true
}
