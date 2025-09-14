use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, r): (usize, usize),
        ll: [u8; n],
    }

    let left_opened_doors = (0..r).filter(|&pos| ll[pos] == 0).collect_vec();
    let right_opened_doors = (r..n).filter(|&pos| ll[pos] == 0).collect_vec();

    let ans = match (left_opened_doors.is_empty(), right_opened_doors.is_empty()) {
        (true, true) => 0,
        (true, false) => {
            let most_right_opened_door = *right_opened_doors.last().unwrap();
            let dist = most_right_opened_door + 1 - r;
            2 * dist - right_opened_doors.len()
        }
        (false, true) => {
            let most_left_opened_door = *left_opened_doors.first().unwrap();
            let dist = r - most_left_opened_door;
            2 * dist - left_opened_doors.len()
        }
        (false, false) => {
            let most_left_opened_door = *left_opened_doors.first().unwrap();
            let left_dist = r - most_left_opened_door;
            let left_cost = 2 * left_dist - left_opened_doors.len();

            let most_right_opened_door = *right_opened_doors.last().unwrap();
            let right_dist = most_right_opened_door + 1 - r;
            let right_cost = 2 * right_dist - right_opened_doors.len();

            left_cost + right_cost
        }
    };
    println!("{ans}");
}
