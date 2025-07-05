use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let mut history = vec![HistoryNode::default()];
    let mut server = 0_usize;
    let mut computers = vec![0_usize; n];

    let queries = (0..q).map(|_| Query::read());
    for query in queries {
        match query {
            Query::Pull(p) => {
                computers[p] = server;
            }
            Query::Append { p, s } => {
                history.push(HistoryNode::new(computers[p], s));
                computers[p] = history.len() - 1;
            }
            Query::Push(p) => {
                server = computers[p];
            }
        }
    }

    let mut strings = vec![history[server].last_string.clone()];
    while let Some(previous) = history[server].previous {
        strings.push(history[previous].last_string.clone());
        server = previous;
    }
    strings.reverse();

    println!("{}", strings.join(""));
}

#[derive(Debug, Clone)]
enum Query {
    Pull(usize),
    Append { p: usize, s: String },
    Push(usize),
}

impl Query {
    fn read() -> Self {
        input! {
            qt: usize,
            p: Usize1,
        }

        match qt {
            1 => Query::Pull(p),
            2 => {
                input! {
                    s: String
                }

                Query::Append { p, s }
            }
            3 => Query::Push(p),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct HistoryNode {
    previous: Option<usize>,
    last_string: String,
}

impl HistoryNode {
    fn new(previous: usize, last_string: String) -> Self {
        Self {
            previous: Some(previous),
            last_string,
        }
    }
}
