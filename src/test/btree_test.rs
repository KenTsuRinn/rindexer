#[cfg(test)]
mod btree_test {
    use crate::reader::btree::BTreeNode;

    #[test]
    fn test_btree_init() {
        let mut node = BTreeNode::new(0, [0x68, 0x65]);
        node.insert_raw(1, [0x6c, 0x6c]);
        node.insert_raw(1, [0x6c, 0x6f]);

        assert_eq!(node.get_value(), [0x68, 0x65]);
        assert_eq!(node.get_right().unwrap().get_right().unwrap().get_value(), [0x68, 0x65]);
        assert_eq!(node.get_right().unwrap().get_value(), [0x68, 0x65]);
    }
}