use std::cell::{Ref, RefCell};
use std::sync::Arc;

type ByteArr = [u8; 2];

pub struct BTreeNode {
    line: u8,
    value: ByteArr,
    left: Option<Arc<RefCell<BTreeNode>>>,
    right: Option<Arc<RefCell<BTreeNode>>>,
}

impl BTreeNode {
    pub fn new(line: u8, value: ByteArr) -> BTreeNode {
        BTreeNode {
            line,
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert_node(&mut self, node: BTreeNode) {
        if self.calculate_value() < node.calculate_value() {
            match self.left.unwrap().borrow_mut() {
                None => self.left = Some(Arc::new(RefCell::new(node))),
                Some(left) => left.insert_node(node)
            }
        } else {
            match self.right.unwrap().borrow_mut() {
                None => self.right = Some(Arc::new(RefCell::new(node))),
                Some(right) => right.insert_node(node)
            }
        }
    }
    pub fn insert_raw(&mut self, line: u8, value: ByteArr) {
        if self.calculate_value() < value.into_iter().sum() {
            match self.left.unwrap().borrow_mut() {
                None => self.left = Some(Arc::new(RefCell::new(BTreeNode {
                    line,
                    value,
                    left: None,
                    right: None,
                }))),
                Some(left) => left.insert_raw(line, value)
            }
        } else {
            match self.right.unwrap().borrow_mut() {
                None => self.right = Some(Arc::new(RefCell::new(BTreeNode {
                    line,
                    value,
                    left: None,
                    right: None,
                }))),
                Some(right) => right.insert_raw(line, value)
            }
        }
    }

    pub fn search(&self, arr: ByteArr) -> Option<ByteArr> {
        let target: u8 = arr.iter().sum();
        let result = match self.calculate_value() {
            value if target == value => Some(self.value),
            value if target < value => Some(self.left.unwrap().borrow().search(arr)),
            value if target > value => Some(self.right.unwrap().borrow().search(arr)),
            _ => None
        };
        result
    }

    pub fn get_left(&self) -> Ref<BTreeNode> {
        self.left.unwrap().borrow()
    }

    pub fn get_right(&self) -> Ref<BTreeNode> {
        self.right.unwrap().borrow()
    }

    pub fn get_value(&self) -> ByteArr {
        self.value
    }

    fn calculate_value(&self) -> u8 {
        self.value.iter().sum()
    }
}
