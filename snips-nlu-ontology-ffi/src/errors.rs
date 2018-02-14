error_chain! {
    types {
        OntologyError, OntologyErrorKind, OntologyResultExt, OntologyResult;
    }

    foreign_links {
        Utf8Error(::std::str::Utf8Error);
        NulError(::std::ffi::NulError);
    }
}
