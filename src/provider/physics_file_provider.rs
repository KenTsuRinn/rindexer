use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};
use std::path::Path;
use crate::model::iprovider::IProvider;

pub struct PhysicsFileProvider {
    path: Box<Path>,
    buffer: Option<String>,
}

impl PhysicsFileProvider {
    pub fn new(path: &str) -> PhysicsFileProvider {
        if path.is_empty() {
            panic!("invalid path.")
        }

        return PhysicsFileProvider {
            path: Box::from(Path::new(path)),
            buffer: None,
        };
    }

    fn create_buffer(&self) -> String {
        if !self.path.exists() {
            panic!("file not exists.")
        }

        let file = File::open(self.path.as_ref()).expect("file not found.");
        let mut content = String::new();
        BufReader::new(file).read_to_string(&mut content);
        return content;
    }
}

impl IProvider for PhysicsFileProvider {
    fn contents(&self) -> String {
        if !self.path.exists() {
            panic!("file not found.");
        }
        let mut file = File::open(self.path.as_ref()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }
}
