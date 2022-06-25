struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct MinStack {
    head: Link<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, val: i32) {
        let min_val = self
            .head
            .as_deref()
            .map_or(val, |node| node.elem.1.min(val));
        let new_node = Box::new(Node {
            elem: (val, min_val),
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next;
        }
    }

    fn top(&self) -> i32 {
        self.head.as_ref().map(|node| node.elem.0).unwrap()
    }

    fn get_min(&self) -> i32 {
        self.head.as_ref().map(|node| node.elem.1).unwrap()
    }
}
