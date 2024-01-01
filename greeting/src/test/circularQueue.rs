struct CircularQueue {
    buffer: Vec<i32>,
    head: usize,
    tail: usize,
}

impl CircularQueue {
    fn new(len: i32) -> Self {
        CircularQueue {
            buffer: vec![0; len as usize + 1],
            head: 0,
            tail: 0,
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.buffer[self.head]
    }

    fn back(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let tail_index = (self.tail + self.buffer.len() - 1) % self.buffer.len();
        self.buffer[tail_index]
    }

    fn push(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.buffer[self.tail] = value;
        self.tail = (self.tail + 1) % self.buffer.len();
        true
    }
    
    fn pop(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.buffer.len();
        true
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn is_full(&self) -> bool {
        (self.tail + 1) % self.buffer.len() == self.head
    }
}

fn main() {
    let mut queue = CircularQueue::new(3);
    queue.push(1);
    queue.push(2);
    queue.push(3);
    println!("{}", queue.push(4));
    println!("{}", queue.back());
    println!("{}", queue.is_full());
    println!("{}", queue.pop());
    println!("{}", queue.push(4));
    println!("{}", queue.back());
}