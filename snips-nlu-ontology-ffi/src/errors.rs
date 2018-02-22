error_chain! {
    types {
        OntologyError, OntologyErrorKind, OntologyResultExt, OntologyResult;
    }

    links {
        OntologyLib(::snips_nlu_ontology::errors::Error, ::snips_nlu_ontology::errors::ErrorKind);
    }

    foreign_links {
        Utf8Error(::std::str::Utf8Error);
        NulError(::std::ffi::NulError);
        Serde(::serde_json::Error);
    }
}
