use proconio::{input, marker::Chars};

fn main() {
    input! {
        (x, y, z): (usize, usize, usize),
        s: Chars,
    }

    let ns_nc = x.min(z + y + z);
    let ns_c = (x + z).min(z + y);
    let s_nc = y.min(z + x + z);
    let s_c = (z + x).min(y + z);

    let mut unlocked = 0;
    let mut locked = z;

    for &c in &s {
        if c == 'a' {
            let next_unlocked = (unlocked + ns_nc).min(locked + s_c);
            let next_locked = (unlocked + ns_c).min(locked + s_nc);

            unlocked = next_unlocked;
            locked = next_locked;
        } else {
            let next_unlocked = (unlocked + s_nc).min(locked + ns_c);
            let next_locked = (unlocked + s_c).min(locked + ns_nc);

            unlocked = next_unlocked;
            locked = next_locked;
        }
    }

    let ans = unlocked.min(locked);
    println!("{}", ans);
}
