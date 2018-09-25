extern crate libc;
extern crate failure;
#[macro_use]
extern crate ffi_utils;
extern crate snips_nlu_ontology;
#[macro_use]
extern crate snips_nlu_ontology_ffi_macros;

generate_error_handling!(snips_nlu_ontology_get_last_error);

export_nlu_ontology_c_symbols!();
