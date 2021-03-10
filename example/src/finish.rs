use std::fmt;

struct Node {
    data: char,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: char, next: Option<Box<Node>>) -> Node {
        Node { data, next }
    }
}

struct Stack {
    head: Option<Box<Node>>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { head: None }
    }

    pub fn push(&mut self, data: char) {
        let new_node = Box::new(Node::new(data, self.head.take()));
        self.head = Some(new_node);
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.data);
                    current = &node.next;
                }
                None => break,
            }
        }
        write!(f, "{}\n", result)
    }
}

fn main() {
    let mut list: Stack = Stack::new();
    let world: [char; 11] = ['d', 'l', 'r', 'o', 'W', ' ', 'o', 'l', 'l', 'e', 'H'];
    for chr in world.iter() {
        list.push(*chr)
    }
    print! {"{}", list}
}
