pub trait IReader<T> {
    fn get_nodes(&self) -> Vec<T>;
}
