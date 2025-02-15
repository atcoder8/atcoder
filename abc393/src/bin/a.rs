use proconio::input;

fn main() {
    input! {
        ss: [String; 2],
    }

    let ans = match (&ss[0] == "sick", &ss[1] == "sick") {
        (true, true) => 1,
        (true, false) => 2,
        (false, true) => 3,
        (false, false) => 4,
    };
    println!("{}", ans);
}
