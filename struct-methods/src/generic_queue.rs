
pub struct GenericQueue<T> {
    older: Vec<T>, // older elements, eldest last
    younger: Vec<T>  // younger elements, youngest last
}

impl<T> GenericQueue<T> {

    /// create a new instance of Queue
    pub fn new() -> Self {
        GenericQueue {
            older: Vec::new(), younger: Vec::new()
        }
    }

    /// push character onto the back of the queue
    pub fn push(&mut self, c: T) {
        self.younger.push(c)
    }

    /// Pop a character of the front of the queue. Return Some(c) if the element exists
    /// or None if the queue is empty
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            // Bring the elements from younger to older and put them in the promised order
            use::std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        // older is guaranteed to have an element inside of it and the vectors pop method
        // returns Option
        self.older.pop()
    }

    /// split the queue inot the older and younger vectors
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }

    /// check if the queue contains any elements
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}