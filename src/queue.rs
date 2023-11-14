use std::{cell::RefCell, collections::VecDeque, ops::Deref, rc::Rc};

struct Queue<T>(VecDeque<T>);

impl<T> Queue<T> {
    fn new() -> Self {
        Queue(VecDeque::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn head(&self) -> Option<&T> {
        self.0.front()
    }

    fn tail(&self) -> Option<&T> {
        self.0.back()
    }

    pub fn peek(&self) -> Option<&T> {
        self.head()
    }

    pub fn deque(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    pub fn enqueue(&mut self, item: T) {
        self.0.push_back(item)
    }
}

mod test {
    use super::Queue;

    #[test]
    fn queue() {
        let mut list: Queue<usize> = Queue::new();

        list.enqueue(3);
        list.enqueue(7);
        list.enqueue(9);

        assert_eq!(list.deque(), Some(3));
        assert_eq!(list.len(), 2);

        list.enqueue(11);
        assert_eq!(list.deque(), Some(7));
        assert_eq!(list.deque(), Some(9));
        assert_eq!(list.peek(), Some(&11));
        assert_eq!(list.deque(), Some(11));
        assert_eq!(list.deque(), None);

        list.enqueue(69);
        assert_eq!(list.peek(), Some(&69));
        assert_eq!(list.len(), 1);
    }
}
