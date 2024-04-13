use itertools::{chain, Itertools};
use proconio::input;

fn main() {
    input! {
        (mut l, mut r): (usize, usize),
    }

    let mut left_sections: Vec<(usize, usize)> = vec![];
    let mut right_sections: Vec<(usize, usize)> = vec![];
    for i in 0..=60 {
        if l == r {
            break;
        }

        let raised = 1_usize << i;

        if l % 2 == 1 {
            left_sections.push((l * raised, (l + 1) * raised));
            l += 1;
        }

        if r % 2 == 1 {
            right_sections.push(((r - 1) * raised, r * raised));
            r -= 1;
        }

        l >>= 1;
        r >>= 1;
    }

    let sections = chain!(left_sections, right_sections.into_iter().rev())
        .sorted_unstable()
        .collect_vec();

    println!(
        "{}\n{}",
        sections.len(),
        sections
            .iter()
            .map(|(l, r)| format!("{} {}", l, r))
            .join("\n")
    );
}
