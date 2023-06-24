use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let t: Vec<char> = ss[i].iter().chain(&ss[j]).cloned().collect();
            if (0..(t.len() / 2)).all(|i| t[i] == t[t.len() - 1 - i]) {
                println!("Yes");
                std::process::exit(0);
            }
        }
    }

    println!("No");
}
