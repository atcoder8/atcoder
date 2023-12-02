use proconio::input;

fn main() {
    input! {
        (m, d): (usize, usize),
        (mut year, mut month, mut day): (usize, usize, usize),
    }

    day += 1;

    if day == d + 1 {
        day = 1;
        month += 1;
    }

    if month == m + 1 {
        month = 1;
        year += 1;
    }

    println!("{} {} {}", year, month, day);
}
