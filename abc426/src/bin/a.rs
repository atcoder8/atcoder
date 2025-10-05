use proconio::input;

const VERSIONS: [&str; 3] = ["Ocelot", "Serval", "Lynx"];

fn main() {
    input! {
        (x, y): (String, String),
    }

    let pos_x = VERSIONS.iter().position(|&version| version == &x);
    let pos_y = VERSIONS.iter().position(|&version| version == &y);
    println!("{}", if pos_x >= pos_y { "Yes" } else { "No" });
}
