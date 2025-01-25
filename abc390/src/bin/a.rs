use proconio::input;

fn main() {
    input! {
        aa: [u8; 5],
    }

    let is_ok = |p: usize| {
        let mut swapped = aa.clone();
        swapped.swap(p, p + 1);
        swapped == [1, 2, 3, 4, 5]
    };

    println!("{}", if (0..4).any(is_ok) { "Yes" } else { "No" });
}
