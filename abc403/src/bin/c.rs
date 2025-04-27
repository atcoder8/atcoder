use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (n, _m, q): (usize, usize, usize),
    }

    let queries = (0..q).map(|_| Query::read()).collect_vec();
    let mut individual_permissions = vec![BTreeSet::<usize>::new(); n];
    let mut all_authorized = vec![false; n];
    for &query in &queries {
        match query {
            Query::GrantOne { user, page } => {
                individual_permissions[user].insert(page);
            }
            Query::GrantAll { user } => all_authorized[user] = true,
            Query::Permission { user, page } => {
                println!(
                    "{}",
                    if all_authorized[user] || individual_permissions[user].contains(&page) {
                        "Yes"
                    } else {
                        "No"
                    }
                );
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    GrantOne { user: usize, page: usize },
    GrantAll { user: usize },
    Permission { user: usize, page: usize },
}

impl Query {
    fn read() -> Self {
        input! {
            qt: usize,
        }

        match qt {
            1 => {
                input! {
                    (x, y): (Usize1, Usize1),
                }

                Query::GrantOne { user: x, page: y }
            }
            2 => {
                input! {
                    x: Usize1,
                }

                Query::GrantAll { user: x }
            }
            3 => {
                input! {
                    (x, y): (Usize1, Usize1),
                }

                Query::Permission { user: x, page: y }
            }
            _ => panic!(),
        }
    }
}
