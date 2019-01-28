use ffi_utils::{generate_error_handling, wrap};
use snips_nlu_ontology_ffi_macros::export_nlu_ontology_c_symbols;

generate_error_handling!(snips_nlu_ontology_get_last_error);

export_nlu_ontology_c_symbols!();
