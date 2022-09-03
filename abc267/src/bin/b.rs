fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    let cc: Vec<char> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().chars().collect()
    };

    let cols = vec![
        vec![6],
        vec![3],
        vec![1, 7],
        vec![0, 4],
        vec![2, 8],
        vec![5],
        vec![9],
    ];

    if cc[0] == '1' {
        return false;
    }

    for i in 0..(cols.len() - 2) {
        if cols[i].iter().all(|&x| cc[x] == '0') {
            continue;
        }

        for j in (i + 2)..cols.len() {
            if cols[j].iter().all(|&x| cc[x] == '0') {
                continue;
            }

            for k in (i + 1)..j {
                if cols[k].iter().all(|&x| cc[x] == '0') {
                    return true;
                }
            }
        }
    }

    false
}
