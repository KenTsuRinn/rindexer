use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::model::iprovider::IProvider;

pub struct PhysicsFileProvider<'pro> {
    path: &'pro Path,
}

impl<'pro> PhysicsFileProvider<'pro> {
    pub fn new(path: &'pro str) -> PhysicsFileProvider<'pro> {
        if path.is_empty() {
            panic!("invalid path.")
        }

        let p: &'pro Path = Path::new(path);
        return PhysicsFileProvider {
            path: p,
        };
    }
}

impl<'pro> IProvider for PhysicsFileProvider<'pro> {
    fn contents(&self) -> String {
        if !self.path.exists() {
            panic!("file not found.");
        }
        let mut file = File::open(self.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }
}