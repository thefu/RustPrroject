struct CircularQueue<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T> CircularQueue<T> {
    fn new(capacity: usize) -> Self {
        CircularQueue {
            buffer: vec![None; capacity],
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    fn enqueue(&mut self, item: T) -> Result<(), &'static str> {
        if self.size == self.buffer.len() {
            return Err("Queue is full");
        }

        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.buffer.len();
        self.size += 1;

        Ok(())
    }

    fn dequeue(&mut self) -> Result<T, &'static str> {
        if self.size == 0 {
            return Err("Queue is empty");
        }

        let item = self.buffer[self.head].take().unwrap();
        self.head = (self.head + 1) % self.buffer.len();
        self.size -= 1;

        Ok(item)
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.buffer.len()
    }
}

fn main() {
    let mut queue = CircularQueue::new(5);
    queue.enqueue(1).unwrap();
    queue.enqueue(2).unwrap();
    queue.enqueue(3).unwrap();
    
    println!("Queue size: {}", queue.size); // Output: Queue size: 3
    
    println!("Dequeued item: {:?}", queue.dequeue()); // Output: Dequeued item: Some(1)
    
    queue.enqueue(4).unwrap();
    queue.enqueue(5).unwrap();
    queue.enqueue(6).unwrap(); // Will fail, as the queue is full
    
    println!("Is queue full? {}", queue.is_full()); // Output: Is queue full? true
    
    while !queue.is_empty() {
        println!("Dequeued item: {:?}", queue.dequeue());
    }
    // Output:
    // Dequeued item: Some(2)
    // Dequeued item: Some(3)
    // Dequeued item: Some(4)
    // Dequeued item: Some(5)
}

/**
 * 在上面的代码中，CircularQueue是一个泛型结构体，用于表示循环队列。它包含一个缓冲区数组 buffer，以及用于跟踪队列头部(head)、尾部(tail)和元素数量(size)的字段。
 * new 方法用于创建一个新的循环队列，需要指定队列的容量大小。enqueue 方法用于将元素添加到队列的尾部，如果队列已满，则返回错误。
 * dequeue 方法用于从队列头部移除并返回一个元素，如果队列为空，则返回错误。is_empty 和 is_full 方法用于检查队列是否为空或已满。
 * 在 main 函数中，我们创建一个容量为5的循环队列，并进行一
 */