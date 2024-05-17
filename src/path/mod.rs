use std::path::{Path, PathBuf};

pub trait PathExt {
    fn here_with_ext(&self, extension: &str) -> PathBuf;
}

impl PathExt for PathBuf {
    fn here_with_ext(&self, extension: &str) -> PathBuf {
        let mut buf = PathBuf::new();
        buf.set_file_name(self.file_name().unwrap());
        buf.with_extension(extension)
    }
}

impl PathExt for &Path {
    fn here_with_ext(&self, extension: &str) -> PathBuf {
        let mut buf = PathBuf::new();
        buf.set_file_name(self.file_name().unwrap());
        buf.with_extension(extension)
    }
}
