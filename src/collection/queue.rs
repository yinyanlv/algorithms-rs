use std::rc::Rc;
use std::cell::RefCell;

struct Node<T> {
    item: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn get_next(&self) -> Option<Rc<RefCell<Node<T>>>> {

        match &self.next {
            Some(next_node) => {
                Some(next_node.clone())
            }
            None => None
        }
    }

    pub fn set_next(&mut self, node: Node<T>) {

        self.next = Some(Rc::new(RefCell::new(node)));
    }

    pub fn has_next(&self) -> bool {
        match &self.next {
            Some(_) => {
                true
            }
            None => false
        }
    }
}

pub struct Queue<T> {
    first: Option<Rc<RefCell<Node<T>>>>,
    size: u64,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            first: None,
            size: 0,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        let node = Node {
            item,
            next: None
        };

        match &mut self.first {
            Some(first_node) => {

                let mut temp_node = first_node.clone();

                while temp_node.borrow().has_next() {
                    temp_node = temp_node.clone().borrow().get_next().unwrap();
                }

                temp_node.borrow_mut().set_next(node);
            }
            None => {

                self.first = Some(Rc::new(RefCell::new(node)));
            }
        };

        self.size += 1;
    }

    pub fn dequeue() -> Option<T> {

        None
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
    #[test]
    fn test_queue() {

    }
}