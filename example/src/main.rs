struct Node {
    data: char,
    next: Option<Box<Node>>,
}

impl Node {
    // functions
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
        self.head
    }
}

fn main() {}
