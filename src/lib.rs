#[macro_use]
extern crate failure;
extern crate itertools;
#[cfg(feature = "builtin_entities")]
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "builtin_entities")]
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
#[cfg(feature = "builtin_entities")]
pub mod builtin_entities;
pub mod language;

mod ontology;

#[cfg(feature = "builtin_entities")]
pub use builtin_entities::*;
pub use ontology::*;
pub use language::*;

pub static ONTOLOGY_VERSION: &str = "0.6.0";
