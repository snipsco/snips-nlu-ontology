#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[cfg(test)]
extern crate serde_test;

#[macro_use]
pub mod macros;
pub mod errors;
pub mod language;

pub mod builtin_entity;
pub mod ontology;

pub use builtin_entity::{BuiltinEntity, BuiltinEntityKind};
pub use ontology::*;
pub use language::*;

pub static ONTOLOGY_VERSION: &str = "0.6.0";
