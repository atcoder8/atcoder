fn main() {
    if let Some((x, y)) = solve() {
        println!("{} {}", x + 1, y + 1);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<(usize, usize)> {
    let (h, w) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let mut ggg: Vec<Vec<char>> = vec![];
    for _ in 0..h {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        ggg.push(line.trim().chars().collect());
    }

    let mut visited = vec![vec![false; w]; h];
    let (mut x, mut y) = (0, 0);

    loop {
        if visited[x][y] {
            return None;
        }

        visited[x][y] = true;

        match ggg[x][y] {
            'U' => {
                if x == 0 {
                    return Some((x, y));
                }

                x -= 1;
            },
            'D' => {
                if x == h - 1 {
                    return Some((x, y));
                }

                x += 1;
            },
            'L' => {
                if y == 0 {
                    return Some((x, y));
                }

                y -= 1;
            },
            'R' => {
                if y == w - 1 {
                    return Some((x, y));
                }

                y += 1;
            },
            _ => panic!()
        }
    }
}
