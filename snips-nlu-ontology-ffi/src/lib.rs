#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate snips_nlu_ontology;

pub mod errors;
#[macro_use]
pub mod macros;

#[macro_use]
mod ffi_utils;
#[cfg(feature = "builtin_entities")]
mod builtin_entity_parser;
#[cfg(feature = "builtin_entities")]
mod builtin_entity;
mod failure_ext;
mod ontology;
mod language;

use snips_nlu_ontology::*;

pub use errors::*;
pub use ontology::*;
pub use language::*;
#[cfg(feature = "builtin_entities")]
pub use builtin_entity::*;
#[cfg(feature = "builtin_entities")]
pub use builtin_entity_parser::*;
pub use ffi_utils::*;

#[no_mangle]
pub extern "C" fn nlu_ontology_version() -> *const libc::c_char {
    ::std::ffi::CString::new(::ONTOLOGY_VERSION)
        .unwrap()
        .into_raw()
}
