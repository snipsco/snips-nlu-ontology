error_chain! {
    links {
        Rustling(::rustling_ontology::RustlingError, ::rustling_ontology::RustlingErrorKind);
    }

    foreign_links {
    }
}
