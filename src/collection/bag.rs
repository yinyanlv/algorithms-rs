use std::rc::Rc;

#[allow(dead_code)]
struct Node<T: Clone> {
    item: T,
    next: Rc<Option<Node<T>>>,
}

pub struct Bag<T: Clone> {
    first: Rc<Option<Node<T>>>,
    size: u64,
}

impl<T: Clone> Bag<T> {
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

pub struct BagIntoIterator<T: Clone> {
    bag: Bag<T>,
    cursor: Rc<Option<Node<T>>>,
}

impl<T: Clone> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = BagIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        BagIntoIterator {
            cursor: self.first.clone(),
            bag: self,
        }
    }
}

impl<T: Clone> Iterator for BagIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {

        if Rc::get_mut(&mut self.cursor).is_none() {
            return None;
        }

        let node_wrap = Rc::get_mut(&mut self.cursor).unwrap();
        match node_wrap {
            Some(node) => {
                let temp;
                {
                    temp = node.item.clone();
                }
                self.cursor = node.next.clone();
                Some(temp)
            }
            None => None
        }
    }
}

#[test]
fn test_bag() {
    let mut bag = Bag::new();

    bag.add(0);
    bag.add(1);
    bag.add(2);

    for item in bag {
        println!("{}", "*****");
        println!("{}", item);
    }

//    assert_eq!(bag.size(), 3);
//    assert_eq!(bag.is_empty(), false);
}