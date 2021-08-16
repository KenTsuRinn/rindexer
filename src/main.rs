use crate::model::{iprovider, ireader};
use crate::model::node::Node;
use crate::provider::physics_file_provider::PhysicsFileProvider;
use crate::reader::node_reader::U8ByteReader;

mod model;
mod provider;
mod reader;
mod test;

fn main() {
    let mut provider: Box<dyn iprovider::IProvider> = Box::from(PhysicsFileProvider::new("./data/health.txt"));
    let reader: &dyn ireader::IReader<Node<u8>> = &U8ByteReader::new(provider);
    let nodes = reader.get_nodes();
}
