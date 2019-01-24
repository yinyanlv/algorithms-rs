use std::rc::Rc;

struct Node<T: Clone> {
    item: T,
    next: Option<Rc<Node<T>>>,
}

impl<T: Clone> Node<T> {
    pub fn get_next(&self) -> Option<Rc<Node<T>>> {
        match &self.next {
            Some(next_node) => Some(next_node.clone()),
            None => None
        }
    }
}

pub struct Bag<T: Clone> {
    first: Option<Rc<Node<T>>>,
    size: u64,
}

impl<T: Clone> Bag<T> {
    pub fn new() -> Self {
        Bag {
            first: None,
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
        match &mut self.first {
            Some(old_first) => {
                let node = Node {
                    item,
                    next: Some(old_first.clone()),
                };

                self.first = Some(Rc::new(node));
            }
            None => {
                let node = Node {
                    item,
                    next: None,
                };

                self.first = Some(Rc::new(node));
            }
        };

        self.size += 1;
    }
}

#[allow(dead_code)]
pub struct BagIntoIterator<T: Clone> {
    bag: Bag<T>,
    cursor: Option<Rc<Node<T>>>,
}

impl<T: Clone> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = BagIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        let cursor;

        match &self.first {
            Some(old_first) => {
                cursor = Some(old_first.clone());
            }
            None => {
                cursor = None;
            }
        };

        BagIntoIterator {
            cursor,
            bag: self,
        }
    }
}

impl<T: Clone> Iterator for BagIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match &mut self.cursor {
            Some(_) => {
                let node = self.cursor.clone().unwrap();

                self.cursor = node.get_next();

                Some(node.item.clone())
            }
            None => None,
        }
    }
}

#[test]
fn test_bag() {
    let mut bag = Bag::new();
    let mut count = 0;

    bag.add(0);
    bag.add(1);
    bag.add(2);

    assert_eq!(bag.size(), 3);
    assert_eq!(bag.is_empty(), false);

    for _ in bag {
        count += 1;
    }

    assert_eq!(count, 3);
}