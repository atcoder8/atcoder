use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aaa: [[Usize1]; m],
        bb: [Usize1; n],
    }

    let mut ingredient_to_dishes = vec![vec![]; n];
    for (dish, ingredients) in enumerate(&aaa) {
        for &ingredient in ingredients {
            ingredient_to_dishes[ingredient].push(dish);
        }
    }

    let mut cnt = 0_usize;
    let mut rem_by_dish = aaa.iter().map(|aa| aa.len()).collect_vec();
    for &b in &bb {
        for &dish in &ingredient_to_dishes[b] {
            rem_by_dish[dish] -= 1;
            if rem_by_dish[dish] == 0 {
                cnt += 1;
            }
        }

        println!("{}", cnt);
    }
}
