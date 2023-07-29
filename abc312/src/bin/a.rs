use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let v = vec!["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];
    println!("{}", if v.contains(&&s[..]) { "Yes" } else { "No" });
}
