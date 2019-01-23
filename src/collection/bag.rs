use std::rc::Rc;

#[allow(dead_code)]
struct Node<T> {
    item: T,
    next: Rc<Option<Node<T>>>,
}

pub struct Bag<T> {
    first: Rc<Option<Node<T>>>,
    size: u64,
}

impl<T> Bag<T> {
    pub fn new() -> Self {
        Bag {
            first: Rc::new(None),
            size: 0,
        }
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn add(&mut self, item: T) {
        let old_first = self.first.clone();

        let node = Node {
            item,
            next: old_first,
        };

        self.first = Rc::new(Some(node));
        self.size += 1;
    }
}

#[test]
fn test_bag() {
    let mut bag = Bag::new();

    bag.add(0);
    bag.add(1);
    bag.add(2);

    assert_eq!(bag.size(), 3);
    assert_eq!(bag.is_empty(), false);
}