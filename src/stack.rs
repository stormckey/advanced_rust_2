use std::cell::RefCell;

struct SimpleStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> SimpleStack<T> {
    fn new() -> SimpleStack<T> {
        SimpleStack {
            stack: RefCell::new(Vec::new()),
        }
    }
    fn push(&self, v: T) {
        self.stack.borrow_mut().push(v);
    }
    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_stack() {
        let stack = SimpleStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(2), stack.pop());
        stack.push(4);
        assert_eq!(Some(4), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }
}
