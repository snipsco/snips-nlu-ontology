error_chain! {
    links {
        Rustling(::rustling_ontology::RustlingError, ::rustling_ontology::RustlingErrorKind) #[cfg(feature = "builtin_entities")];
    }

    foreign_links {
        Serde(::serde_json::Error);
    }
}
