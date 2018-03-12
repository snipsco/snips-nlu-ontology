#[macro_use]
extern crate failure;
extern crate itertools;
#[cfg(feature = "parsers")]
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "parsers")]
extern crate rustling_ontology;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[cfg(test)]
extern crate serde_test;

#[macro_use]
pub mod macros;
pub mod errors;
#[cfg(feature = "parsers")]
pub mod parsers;
pub mod language;

pub mod builtin_entity;
mod ontology;

#[cfg(feature = "parsers")]
pub use parsers::*;
pub use builtin_entity::{BuiltinEntity, BuiltinEntityKind};
pub use ontology::*;
pub use language::*;

pub static ONTOLOGY_VERSION: &str = "0.6.0";
