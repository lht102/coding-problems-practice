struct MyCircularQueue {
    q: Vec<i32>,
    head: usize,
    cnt: usize,
    sz: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let k = k as usize;
        Self {
            q: vec![0; k],
            head: 0,
            cnt: 0,
            sz: k,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.q[(self.head + self.cnt) % self.sz] = value;
        self.cnt += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.sz;
        self.cnt -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.q[self.head]
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.q[(self.head + self.cnt - 1) % self.sz]
    }

    fn is_empty(&self) -> bool {
        self.cnt == 0
    }

    fn is_full(&self) -> bool {
        self.cnt == self.sz
    }
}
