extern crate failure;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate snips_nlu_ontology;
#[macro_use]
extern crate snips_nlu_ontology_ffi_macros;
extern crate snips_nlu_ontology_parsers;

type Result<T> = ::std::result::Result<T, ::failure::Error>;

mod builtin_entity_parser;

pub use builtin_entity_parser::*;

use snips_nlu_ontology_ffi_macros::CResult;

export_nlu_ontology_c_symbols!();