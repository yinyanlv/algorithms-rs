pub struct Node<T> {
    item: T,
    next: Box<Option<Node<T>>>,
}

pub struct Bag<T> {
    first: Box<Option<Node<T>>>,
    size: u64,
}

impl<T> Bag<T> {
    fn new() -> Self {
        Bag {
            first: Box::new(None),
            size: 0,
        }
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn add(&mut self, item: T) {
        let old_first = self.first;

        let node = Node {
            item,
            next: Box::new(None)
        };

        self.first = Box::new(Some(node));
        self.size += 1;
    }
}

#[test]
fn test_bag() {
    let mut bag = Bag::new();

    bag.add(0);

    assert_eq!(bag.size(), 1);
    assert_eq!(bag.is_empty(), false);
}