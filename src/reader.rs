pub mod reader {
    use crate::provider::provider::IProvider;

    pub trait IReader<T> {
        fn get_nodes(&self) -> Vec<T>;
    }

    pub struct Node<T> {
        pub line: usize,
        pub bytes: Vec<T>,
    }

    impl Node<u8> {
        pub fn new(line: usize, contents: &str) -> Node<u8> {
            return Node {
                line,
                bytes: contents.chars().map(|c| c as u8).collect::<Vec<u8>>(),
            };
        }
    }


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
}
