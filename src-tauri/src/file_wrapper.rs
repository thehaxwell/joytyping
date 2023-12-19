use std::fs;
use std::path::Path;
use std::{fs::File, io};
use std::io::{Write, ErrorKind};

pub trait FileWrapperTrait {
    fn create_and_write_all(&self,
		path: &Path,
		bytes: &[u8]) -> Result<(),io::Error>;
    //TODO: rename to remove_file
    fn remove_path(&self, path: &Path,) -> Result<(),io::Error>;
    fn create_dir_if_non_existant(&self, path: &Path) -> Result<bool,io::Error>;
}

pub struct FileWrapper;
impl FileWrapperTrait for FileWrapper {
    fn create_and_write_all(&self,
		path: &Path,
		bytes: &[u8]) -> Result<(),io::Error>{
        let mut out = File::create(path)?;
        out.write_all(bytes)
    }

    fn remove_path(&self, path: &Path,) -> Result<(),io::Error> {
        fs::remove_file(path)
    }

    fn create_dir_if_non_existant(&self, path: &Path) -> Result<bool,io::Error> {
        match fs::create_dir(path) {
            Ok(()) => Ok(true),
            Err(err) => match err.kind() {
                ErrorKind::AlreadyExists => Ok(false),
                _other => Err(err),
            }
        }
    }
}
