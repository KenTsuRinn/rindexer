pub struct Node<T> {
    pub line: usize,
    pub bytes: Vec<T>,
}

impl Node<u8> {
    pub fn new(line: usize, contents: String) -> Node<u8> {
        return Node {
            line,
            bytes: contents.chars().map(|c| c as u8).collect::<Vec<u8>>(),
        };
    }
}