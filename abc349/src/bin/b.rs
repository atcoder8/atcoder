use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut counts = vec![0; 26];
    for c in s.chars() {
        counts[char_to_number(c)] += 1;
    }

    let is_ok = |i: usize| {
        let cnt = counts.iter().filter(|&&cnt| cnt == i).count();
        cnt == 0 || cnt == 2
    };

    let ans = (1..=s.len()).all(is_ok);
    println!("{}", if ans { "Yes" } else { "No" });
}

/// Converts the character `c` to the corresponding numeric value.
pub fn char_to_number(c: char) -> usize {
    (c as u8 - b'a') as usize
}
