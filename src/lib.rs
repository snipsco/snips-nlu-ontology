#[macro_use]
extern crate error_chain;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate rustling_ontology;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate serde_json;
#[cfg(test)]
extern crate serde_test;

#[macro_use]
pub mod macros;
pub mod errors;
pub mod builtin_entities;
pub mod language;

mod ontology;

pub use rustling_ontology::Lang as RustlingLang;
pub use builtin_entities::*;
pub use ontology::*;
pub use language::*;

pub static ONTOLOGY_VERSION: &str = "0.6.0";
