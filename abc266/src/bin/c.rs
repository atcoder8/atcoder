fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    let mut vertexes = Vec::new();
    for _ in 0..4 {
        vertexes.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<i64>().unwrap(),
                iter.next().unwrap().parse::<i64>().unwrap(),
            )
        });
    }

    for i in 0..4 {
        let (x1, y1) = vertexes[i];
        let (x2, y2) = vertexes[(i + 1) % 4];
        let (x3, y3) = vertexes[(i + 2) % 4];

        let (diff_x_1, diff_y_1) = (x2 - x1, y2 - y1);
        let (diff_x_2, diff_y_2) = (x3 - x2, y3 - y2);

        let cross = diff_x_1 * diff_y_2 - diff_x_2 * diff_y_1;
        if cross < 0 {
            return false;
        }
    }

    true
}
