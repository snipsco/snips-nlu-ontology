use std::path::{PathBuf, Path};

pub fn test_path() -> PathBuf {
    Path::new("..")
        .join("data")
        .join("tests")
}
