use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        ww: [u64; n],
        q: usize,
    }

    let calc_group_left = |group: AlignedGroup| match group.align_dir {
        Direction::Left => group.align_pos,
        Direction::Right => group.align_pos - ww[group.last_section],
    };

    let calc_group_right = |group: AlignedGroup| match group.align_dir {
        Direction::Left => group.align_pos + ww[group.last_section],
        Direction::Right => group.align_pos,
    };

    let calc_section_left = |group: AlignedGroup, section_length: u64| match group.align_dir {
        Direction::Left => group.align_pos,
        Direction::Right => group.align_pos - section_length,
    };

    let calc_section_right = |group: AlignedGroup, section_length: u64| match group.align_dir {
        Direction::Left => group.align_pos + section_length,
        Direction::Right => group.align_pos,
    };

    let queries = (0..q).map(|_| Query::read()).collect_vec();

    let mut groups = vec![AlignedGroup::new(n - 1, Direction::Left, 0)];
    for query in queries {
        match query {
            Query::Alignment { dir, v } => {
                while groups.last_mut().unwrap().last_section < v {
                    groups.pop();
                }

                let group = groups.last_mut().unwrap();

                let align_pos = match dir {
                    Direction::Left => calc_section_left(*group, ww[v]),
                    Direction::Right => calc_section_right(*group, ww[v]),
                };

                if group.last_section == v {
                    groups.pop();
                }

                groups.push(AlignedGroup::new(v, dir, align_pos));
            }
            Query::Count(x) => {
                let group_pos = groups
                    .partition_point(|group| {
                        calc_group_left(*group) <= x && x < calc_group_right(*group)
                    })
                    .checked_sub(1);

                let Some(group_pos) = group_pos else {
                    println!("0");
                    continue;
                };

                let group = groups[group_pos];

                let first_section = if group_pos + 1 < groups.len() {
                    groups[group_pos + 1].last_section + 1
                } else {
                    0
                };

                let num_exclude_sections = first_section
                    + ww[first_section..=group.last_section].partition_point(|&w| {
                        !(calc_section_left(group, w) <= x && x < calc_section_right(group, w))
                    });

                println!("{}", n - num_exclude_sections);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct AlignedGroup {
    last_section: usize,
    align_dir: Direction,
    align_pos: u64,
}

impl AlignedGroup {
    fn new(last_section: usize, align_dir: Direction, align_pos: u64) -> Self {
        Self {
            last_section,
            align_dir,
            align_pos,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Alignment { dir: Direction, v: usize },
    Count(u64),
}

impl Query {
    fn read() -> Self {
        input! {
            qt: u8,
        }

        match qt {
            1 => {
                input! {
                    v: Usize1,
                }

                Query::Alignment {
                    dir: Direction::Left,
                    v,
                }
            }
            2 => {
                input! {
                    v: Usize1,
                }

                Query::Alignment {
                    dir: Direction::Right,
                    v,
                }
            }
            3 => {
                input! {
                    x: u64,
                }

                Query::Count(x)
            }
            _ => panic!(),
        }
    }
}
