#[cfg(test)]
mod physics_file_reader_test {
    use crate::model::{iprovider, ireader};
    use crate::model::node::Node;
    use crate::provider::physics_file_provider::PhysicsFileProvider;
    use crate::reader::node_reader::U8ByteReader;

    #[test]
    fn test_get_line_vec() {
        let provider: &dyn iprovider::IProvider = &PhysicsFileProvider::new("./data/health.txt".to_string());
        let contents = provider.contents();
        assert_eq!(contents.is_empty(), false);
    }

    #[test]
    fn test_get_nodes() {
        let provider: Box<dyn iprovider::IProvider> = Box::from(PhysicsFileProvider::new("./data/health.txt".to_string()));
        let reader: &dyn ireader::IReader<Node<u8>> = &U8ByteReader::new(provider);
        let nodes = reader.get_nodes();
        assert_eq!(nodes.is_empty(), false);
        assert_eq!(nodes[0].line, 0);
    }
}
