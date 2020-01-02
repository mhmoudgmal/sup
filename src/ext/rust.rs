use std::path::Path;

pub trait PathExt {
    fn does_not_exist(&self) -> bool;
}

impl PathExt for Path {
    fn does_not_exist(&self) -> bool {
        !self.exists()
    }
}
