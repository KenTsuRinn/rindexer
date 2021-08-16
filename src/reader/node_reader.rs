use crate::model::node::Node;
use crate::model::iprovider::IProvider;
use crate::model::ireader::IReader;

fn line_process(num: usize, contents: &str) -> Vec<Node<u8>> {
    let nodes = contents.split(' ').map(|x| {
        return Node::new(num, contents);
    }).collect();
    nodes
}

pub struct U8ByteReader {
    provider: Box<IProvider>,
    line_hooks: Vec<Box<dyn Fn(usize, &str) -> Vec<Node<u8>>>>,
}

impl U8ByteReader {
    pub fn new(provider: Box<IProvider>) -> U8ByteReader {
        return U8ByteReader {
            provider,
            line_hooks: vec![
                Box::from(line_process)
            ],
        };
    }
}

impl IReader<Node<u8>> for U8ByteReader {
    fn get_nodes(&self) -> Vec<Node<u8>> {
        let contents = self.provider.contents();
        let mut line_counter: usize = 0;

        let mut nodes: Vec<Node<u8>> = vec![];
        for line in contents.lines() {
            for hook in &self.line_hooks {
                let mut n = hook(line_counter, line);
                nodes.append(&mut n);
            }
        }

        nodes
    }
}
