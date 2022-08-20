fn main() {
    let mut ss: Vec<char> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().chars().collect()
    };
    let mut tt: Vec<char> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().chars().collect()
    };

    if ss == tt {
        println!("Yes");
        std::process::exit(0);
    }

    if ss.len() > tt.len() {
        println!("No");
        std::process::exit(0);
    }

    while !ss.is_empty() && !tt.is_empty() {
        if ss[ss.len() - 1] != tt[tt.len() - 1] {
            println!("No");
            std::process::exit(0);
        }

        let mut l1 = 1;
        while l1 < ss.len() {
            if ss[ss.len() - l1] == ss[ss.len() - (l1 + 1)] {
                l1 += 1;
            } else {
                break;
            }
        }

        let mut l2 = 1;
        while l2 < tt.len() {
            if tt[tt.len() - l2] == tt[tt.len() - (l2 + 1)] {
                l2 += 1;
            } else {
                break;
            }
        }

        if l1 > l2 || (l1 == 1 && l2 >= 2) {
            println!("No");
            std::process::exit(0);
        }

        for _ in 0..l1 {
            ss.pop();
        }
        for _ in 0..l2 {
            tt.pop();
        }
    }

    if ss.is_empty() && tt.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
