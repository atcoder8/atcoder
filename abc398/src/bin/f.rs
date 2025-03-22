use itertools::chain;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let n = s.len();

    let extended_s = chain(s.chars().rev(), s.chars()).collect::<Vec<char>>();
    let lengths = z_algorithm(&extended_s);
    let common_len = (1..=n)
        .filter(|&len| lengths[2 * n - len] == len)
        .max()
        .unwrap_or(0);
    let (s1, s2) = s.split_at(n - common_len);
    let ans = chain!(s1.chars(), s2.chars(), s1.chars().rev()).collect::<String>();
    println!("{}", ans);
}

/// For each non-negative integer `i` less than `|seq|`,
/// find the length of the longest common prefix of `seq` and `seq[i..]`.
pub fn z_algorithm<T>(seq: &[T]) -> Vec<usize>
where
    T: Eq,
{
    if seq.is_empty() {
        return vec![];
    }

    let n = seq.len();

    let mut lengths = vec![0; n];
    lengths[0] = n;

    let mut cursor = 1;
    let mut common_len = 0;
    while cursor < n {
        while cursor + common_len < n && seq[cursor + common_len] == seq[common_len] {
            common_len += 1;
        }

        if common_len == 0 {
            cursor += 1;
            continue;
        }

        lengths[cursor] = common_len;

        let mut shift = 1;
        while shift + lengths[shift] < common_len {
            lengths[cursor + shift] = lengths[shift];
            shift += 1;
        }

        cursor += shift;
        common_len -= shift;
    }

    lengths
}
