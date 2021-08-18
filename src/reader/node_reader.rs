use crate::model::iprovider::IProvider;
use crate::model::ireader::IReader;
use crate::model::node::Node;

fn line_process<'red>(num: usize, contents: String) -> Vec<Node<u8>> {
    let nodes = contents.split(' ').map(|x| {
        return Node::new(num, x.to_string());
    }).collect();
    nodes
}

pub struct U8ByteReader<'red> {
    provider: &'red dyn IProvider,
    line_hooks: Vec<&'red dyn Fn(usize, String) -> Vec<Node<u8>>>,
}

impl<'red> U8ByteReader<'red> {
    pub fn new(provider: &'red dyn IProvider) -> U8ByteReader<'red> {
        return U8ByteReader {
            provider,
            line_hooks: vec![
                &line_process
            ],
        };
    }
}

impl<'red> IReader<Node<u8>> for U8ByteReader<'red> {
    fn get_nodes(&self) -> Vec<Node<u8>> {
        let contents = self.provider.contents();
        let mut line_counter: usize = 0;

        let mut nodes: Vec<Node<u8>> = vec![];
        for line in contents.lines() {
            for hook in &self.line_hooks {
                let mut n = hook(line_counter, line.to_string());
                line_counter += 1;
                nodes.append(&mut n);
            }
        }

        nodes
    }
}
