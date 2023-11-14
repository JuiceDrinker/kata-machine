use std::collections::VecDeque;

pub struct Stack<T>(VecDeque<T>);

impl<T> Stack<T> {
    fn new() -> Self {
        Stack(VecDeque::new())
    }

    fn head(&self) -> Option<&T> {
        self.0.front()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.head()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_back()
    }

    pub fn push(&mut self, item: T) {
        self.0.push_back(item)
    }
}

mod test {
    use super::Stack;

    #[test]
    fn stack() {
        let mut list: Stack<usize> = Stack::new();
        list.push(5);
        list.push(7);
        list.push(9);

        assert_eq!(list.pop(), Some(9));
        assert_eq!(list.len(), 2);

        list.push(11);

        assert_eq!(list.pop(), Some(11));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.peek(), Some(&5));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        list.push(69);

        assert_eq!(list.peek(), Some(&69));
        assert_eq!(list.len(), (1));
    }
}
