use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut placeable = true;
    let mut t = String::new();
    for ch in s.chars() {
        let add_ch = if ch == '#' {
            placeable = true;
            '#'
        } else if placeable {
            placeable = false;
            'o'
        } else {
            '.'
        };
        t.push(add_ch);
    }

    println!("{}", t);
}
