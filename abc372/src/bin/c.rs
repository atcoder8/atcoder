use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const ABC: [char; 3] = ['A', 'B', 'C'];

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        mut s: Chars,
        xc: [(Usize1, char); q],
    }

    let mut cnt = s.windows(3).filter(|sub| sub == &&ABC).count();
    for &(x, next) in &xc {
        let cur = s[x];
        let cur_offset = char_to_int(cur);
        if ABC.contains(&cur)
            && x >= cur_offset
            && x - cur_offset < n - 2
            && s[x - cur_offset..x - cur_offset + 3] == ABC
        {
            cnt -= 1;
        }

        s[x] = next;

        let next_offset = char_to_int(next);
        if ABC.contains(&next)
            && x >= next_offset
            && x - next_offset < n - 2
            && s[x - next_offset..x - next_offset + 3] == ABC
        {
            cnt += 1;
        }

        println!("{}", cnt);
    }
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'A') as usize
}
