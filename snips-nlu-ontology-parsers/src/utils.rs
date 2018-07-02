use std::path::Path;

pub fn gazetteer_entity_path(name: &str) -> ::std::path::PathBuf {
    Path::new("..")
        .join("data")
        .join("tests")
        .join("gazetteer_entities")
        .join(name)
        .to_path_buf()
}
