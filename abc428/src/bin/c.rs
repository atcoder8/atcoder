use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let queries = (0..q).map(|_| Query::read());
    let mut stack: Vec<char> = vec![];
    let mut level = 0_i64;
    let mut minuses: Vec<bool> = vec![];
    let mut num_minuses = 0_usize;
    for query in queries {
        match query {
            Query::Push(c) => {
                stack.push(c);

                if c == '(' {
                    level += 1;
                } else {
                    level -= 1;
                }

                minuses.push(level < 0);
                num_minuses += (level < 0) as usize;
            }
            Query::Pop => {
                let c = stack.pop().unwrap();
                if c == '(' {
                    level -= 1;
                } else {
                    level += 1;
                }

                num_minuses -= minuses.pop().unwrap() as usize;
            }
        }

        println!(
            "{}",
            if level == 0 && num_minuses == 0 {
                "Yes"
            } else {
                "No"
            }
        );
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Push(char),
    Pop,
}

impl Query {
    fn read() -> Self {
        input! {
            qt: u8,
        }

        if qt == 1 {
            input! {
                c: char,
            }

            Query::Push(c)
        } else {
            Query::Pop
        }
    }
}
