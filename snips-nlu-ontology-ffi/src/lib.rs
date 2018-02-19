#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate snips_nlu_ontology;
extern crate serde;
extern crate serde_json;

pub mod errors;
#[macro_use]
pub mod macros;

#[macro_use]
mod ffi_utils;
mod builtin_entity_parser;
mod builtin_entity;
mod ontology;
mod language;

use snips_nlu_ontology::*;

pub use ontology::*;
pub use language::*;
pub use builtin_entity::*;
pub use builtin_entity_parser::*;
pub use ffi_utils::{
    nlu_ontology_destroy_string_array,
    nlu_ontology_destroy_string,
    nlu_ontology_get_last_error,
    CStringArray
};

#[no_mangle]
pub extern "C" fn nlu_ontology_version() -> *const libc::c_char {
    ::std::ffi::CString::new(::ONTOLOGY_VERSION).unwrap().into_raw()
}
