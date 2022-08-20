use linked_list::List;

fn main() {
    let mut lis = List::new();
    for i in 0..10 {
        lis.add(i);
        println!("{}", lis);
    }
}

pub mod linked_list {
    use std::{
        cell::RefCell,
        fmt::Display,
        rc::{Rc, Weak},
    };

    pub struct Node {
        pub val: i32,
        pub prev: Option<Weak<RefCell<Node>>>,
        pub next: Option<Rc<RefCell<Node>>>,
    }

    pub struct List {
        pub head: Option<Rc<RefCell<Node>>>,
        pub tail: Option<Weak<RefCell<Node>>>,
    }

    impl Display for List {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut chars = String::default();
            if let Some(head) = self.head.clone() {
                let mut node = Rc::clone(&head);
                chars.push_str(&node.borrow().val.to_string());
                while let Some(next_node) = Rc::clone(&node).borrow().next.as_ref() {
                    node = Rc::clone(&next_node);
                    chars.push(' ');
                    chars.push_str(&node.borrow().val.to_string());
                }
            }
            write!(f, "{}", &chars)
        }
    }

    impl List {
        pub fn new() -> Self {
            Self {
                head: None,
                tail: None,
            }
        }

        pub fn add(&mut self, val: i32) {
            match self.head.clone() {
                Some(head) => {
                    let mut node = Rc::clone(&head);
                    while let Some(next_node) = Rc::clone(&node).borrow().next.as_ref() {
                        node = Rc::clone(&next_node);
                    }
                    let next_node = Node {
                        val,
                        prev: Some(Rc::downgrade(&node)),
                        next: None,
                    };
                    node.borrow_mut().next = Some(Rc::new(RefCell::new(next_node)));
                }
                None => {
                    let next_node = Node {
                        val,
                        prev: None,
                        next: None,
                    };
                    let head = Rc::new(RefCell::new(next_node));
                    let tail = Rc::downgrade(&head);
                    self.head = Some(head);
                    self.tail = Some(tail);
                }
            }
        }
    }
}

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn hello(name: &str) {
//     println!("Hello, {}!", name);
// }
