use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (h, w, m): (usize, usize, usize),
        tax: [(usize, Usize1, usize); m],
    }

    let mut counts = vec![0; 2 * 10_usize.pow(5) + 1];

    let mut hor_painted = vec![false; h];
    let mut ver_painted = vec![false; w];
    let mut painted_row_num = 0;
    let mut painted_col_num = 0;
    for &(t, a, x) in tax.iter().rev() {
        if t == 1 {
            if hor_painted[a] {
                continue;
            }

            counts[x] += w - painted_col_num;

            hor_painted[a] = true;
            painted_row_num += 1;
        } else {
            if ver_painted[a] {
                continue;
            }

            counts[x] += h - painted_row_num;

            ver_painted[a] = true;
            painted_col_num += 1;
        }
    }

    counts[0] += h * w - counts.iter().sum::<usize>();

    let ans = enumerate(counts).filter(|&(_, cnt)| cnt != 0).collect_vec();
    println!(
        "{}\n{}",
        ans.len(),
        ans.iter()
            .map(|&(color, cnt)| format!("{} {}", color, cnt))
            .join("\n")
    );
}
