use std::path::Path;

pub fn gazetteer_entity_path(entity_file_name: &str) -> ::std::path::PathBuf {
    Path::new("..")
        .join("data")
        .join("tests")
        .join("gazetteer_entities")
        .join(entity_file_name)
        .to_path_buf()
}
