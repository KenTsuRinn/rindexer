#[cfg(test)]
mod physics_file_reader_test {
    use crate::provider::provider::{IProvider, PhysicsFileProvider};
    use crate::reader::reader::{IReader, U8ByteReader};

    #[test]
    fn test_get_line_vec() {
        let mut provider = PhysicsFileProvider::new("./data/health.txt");
        let contents = provider.contents();
        assert_eq!(contents.is_empty(), false);
    }

    #[test]
    fn test_get_nodes() {
        let mut provider = Box::from(PhysicsFileProvider::new("./data/health.txt"));
        let reader = U8ByteReader::new(provider);
        let nodes = reader.get_nodes();
        assert_eq!(nodes.is_empty(), false);
        assert_eq!(nodes[0].line, 0);
    }
}
