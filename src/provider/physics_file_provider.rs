use std::fs::File;
use std::path::Path;
use crate::model::iprovider::IProvider;
use std::io::Read;

pub struct PhysicsFileProvider {
    path: Box<Path>,
}

impl PhysicsFileProvider {
    pub fn new(path: String) -> PhysicsFileProvider {
        if path.is_empty() {
            panic!("invalid path.")
        }

        return PhysicsFileProvider {
            path: Box::from(Path::new(path.as_str())),
        };
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
