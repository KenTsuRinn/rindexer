use std::sync::Arc;

type ByteArr = [u8; 2];

pub struct BTreeNode {
    line: u8,
    value: ByteArr,
    left: Option<Arc<BTreeNode>>,
    right: Option<Arc<BTreeNode>>,
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
            match self.left.as_mut() {
                None => self.left = Some(Arc::new(node)),
                Some(left) => left.insert_node(node)
            }
        } else {
            match self.right.as_mut() {
                None => self.right = Some(Arc::new(node)),
                Some(right) => right.insert_node(node)
            }
        }
    }
    pub fn insert_raw(&mut self, line: u8, value: ByteArr) {
        if self.calculate_value() < value.into_iter().sum() {
            match self.left.as_mut() {
                None => self.left = Some(Arc::new(BTreeNode {
                    line,
                    value,
                    left: None,
                    right: None,
                })),
                Some(left) => left.insert_raw(line, value)
            }
        } else {
            match self.right.as_mut() {
                None => self.right = Some(Arc::new(BTreeNode {
                    line,
                    value,
                    left: None,
                    right: None,
                })),
                Some(right) => right.insert_raw(line, value)
            }
        }
    }

    pub fn search(&self, arr: ByteArr) -> Option<ByteArr> {
        let target: u8 = arr.iter().sum();
        let result = match self.calculate_value() {
            value if target == value => Some(self.value),
            value if target < value => Some(self.left.unwrap().search(arr).unwrap()),
            value if target > value => Some(self.right.unwrap().search(arr).unwrap()),
            _ => None
        };
        result
    }

    pub fn get_left(&self) -> Option<&Arc<BTreeNode>> {
        self.left.as_ref()
    }

    pub fn get_right(&self) -> Option<&Arc<BTreeNode>> {
        self.right.as_ref()
    }

    pub fn get_value(&self) -> ByteArr {
        self.value
    }

    fn calculate_value(&self) -> u8 {
        self.value.iter().sum()
    }
}
