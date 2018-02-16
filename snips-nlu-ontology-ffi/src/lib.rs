extern crate snips_nlu_ontology;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate libc;

pub mod errors;
#[macro_use]
pub mod macros;

#[macro_use]
mod ffi_utils;
mod builtin_entity_parser;
mod builtin_entity;
mod ontology;
mod language;

use std::ffi::CString;
use snips_nlu_ontology::*;


pub use ffi_utils::{CStringArray, nlu_ontology_destroy_string_array};
pub use ontology::*;
pub use language::*;
pub use builtin_entity::*;
pub use builtin_entity_parser::*;

#[no_mangle]
pub extern "C" fn nlu_ontology_version() -> *const libc::c_char {
    CString::new(::ONTOLOGY_VERSION).unwrap().into_raw()
}
