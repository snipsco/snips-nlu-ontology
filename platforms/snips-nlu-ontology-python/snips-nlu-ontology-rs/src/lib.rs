extern crate failure;
extern crate libc;
extern crate snips_nlu_ontology;
#[macro_use]
extern crate snips_nlu_ontology_ffi_macros;
#[macro_use]
extern crate snips_nlu_ontology_parsers_ffi_macros;
#[macro_use]
extern crate ffi_utils;

generate_error_handling!(snips_nlu_ontology_get_last_error);

export_nlu_ontology_c_symbols!();

export_nlu_ontology_parsers_c_symbols!();
