use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let tree = rec(&s, &mut 0, 0);
    show(&tree);
    println!();
}

#[derive(Debug)]
enum Node {
    Cons(Vec<Node>),
    Char(char),
}

fn rec(s: &[char], idx: &mut usize, level: usize) -> Node {
    let mut children: Vec<Node> = vec![];

    while *idx < s.len() {
        let c = s[*idx];

        match c {
            '(' => {
                *idx += 1;
                let child = rec(s, idx, level + 1);
                children.push(child);
            }

            ')' => {
                *idx += 1;
                break;
            }

            c => {
                if level % 2 == 0 {
                    children.push(Node::Char(c));
                } else {
                    let rev_c = if c.is_ascii_uppercase() {
                        c.to_ascii_lowercase()
                    } else {
                        c.to_ascii_uppercase()
                    };

                    children.push(Node::Char(rev_c));
                }
                *idx += 1;
            }
        }
    }

    if level % 2 == 1 {
        children.reverse();
    }

    Node::Cons(children)
}

fn show(tree: &Node) {
    match tree {
        Node::Cons(children) => {
            for child in children {
                show(child);
            }
        }
        Node::Char(c) => print!("{}", c),
    }
}
