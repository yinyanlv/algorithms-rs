use std::rc::Rc;

struct Node<T: Clone> {
    item: T,
    next: Option<Rc<Node<T>>>,
}

impl<T: Clone> Node<T> {
    pub fn get_next(&self) -> Option<Rc<Node<T>>> {

        match &self.next {
            Some(next_node) => {
                Some(next_node.clone())
            }
            None => None
        }
    }
}

pub struct Stack<T: Clone> {
    first: Option<Rc<Node<T>>>,
    size: u64,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack {
            first: None,
            size: 0,
        }
    }

    pub fn push(&mut self, item: T) {

        match &mut self.first {
            Some(old_first) => {
                let node = Node {
                    item,
                    next: Some(old_first.clone())
                };

                self.first = Some(Rc::new(node));
            }
            None => {
                let node = Node {
                    item,
                    next: None
                };

                self.first = Some(Rc::new(node));
            }
        };

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {

        if self.is_empty() {
            return None;
        }

        let old_first = &mut self.first.clone().unwrap();

        let result = match &old_first.get_next() {
            Some(node) => {
                self.first = Some(node.clone());
                Some(old_first.item.clone())
            }
            None => {
                self.first = None;
                Some(old_first.item.clone())
            }
        };

        self.size -= 1;

        return result;
    }

    pub fn size(&self) -> u64 {

        self.size
    }

    pub fn is_empty(&self) -> bool {

        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {

        let mut stack = Stack::new();

        assert_eq!(stack.is_empty(), true);

        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);

        assert_eq!(stack.size(), 4);
        assert_eq!(stack.is_empty(), false);

        let item1 = stack.pop().unwrap();

        assert_eq!(item1, 4);
        assert_eq!(stack.size(), 3);
    }
}
