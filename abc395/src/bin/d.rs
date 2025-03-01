use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let queries = (0..q).map(|_| Query::read()).collect_vec();

    // 実際の巣の番号から仮想的な巣の番号への対応付け
    let mut real_to_virtual = (0..n).collect_vec();

    // 仮想的な巣の番号から実際の巣の番号への対応付け
    let mut virtual_to_real = (0..n).collect_vec();

    // 各鳩のいる仮想的な巣の番号
    let mut virtual_places = (0..n).collect_vec();

    for query in queries {
        match query {
            Query::Move { pigeon, nest } => {
                virtual_places[pigeon] = real_to_virtual[nest];
            }
            Query::Swap { nest1, nest2 } => {
                let virtual1 = real_to_virtual[nest1];
                let virtual2 = real_to_virtual[nest2];
                virtual_to_real.swap(virtual1, virtual2);

                real_to_virtual.swap(nest1, nest2);
            }
            Query::Report { pigeon } => {
                let virtual_nest = virtual_places[pigeon];
                let nest = virtual_to_real[virtual_nest];
                println!("{}", nest + 1);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Move { pigeon: usize, nest: usize },
    Swap { nest1: usize, nest2: usize },
    Report { pigeon: usize },
}

impl Query {
    fn read() -> Self {
        input! {
            qt: u8,
        }

        match qt {
            1 => {
                input! {
                    (pigeon, nest): (Usize1, Usize1),
                }

                Self::Move { pigeon, nest }
            }

            2 => {
                input! {
                    (nest1, nest2): (Usize1, Usize1),
                }

                Self::Swap { nest1, nest2 }
            }

            3 => {
                input! {
                    pigeon: Usize1,
                }

                Self::Report { pigeon }
            }

            _ => panic!(),
        }
    }
}
