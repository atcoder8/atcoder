use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        ss: [String; n],
    }

    let mut sweet_cnt = 0;
    for s in &ss {
        if sweet_cnt == 2 {
            return false;
        }

        if s == "sweet" {
            sweet_cnt += 1;
        } else {
            sweet_cnt = 0;
        }
    }

    true
}
