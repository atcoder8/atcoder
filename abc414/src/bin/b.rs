use proconio::input;

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "Too Long".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<String> {
    input! {
        n: usize,
        cl: [(char, usize); n],
    }

    let mut ans = String::new();
    for &(c, l) in &cl {
        if ans.len() + l > 100 {
            return None;
        }

        ans.push_str(&c.to_string().repeat(l));
    }

    Some(ans)
}
