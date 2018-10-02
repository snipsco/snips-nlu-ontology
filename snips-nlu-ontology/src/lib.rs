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

pub mod entity;
mod ontology;

pub use entity::builtin_entity::{BuiltinEntity,
                                 BuiltinEntityKind,
                                 IntoBuiltinEntityKind,
                                 language_entity_ontology,
                                 complete_entity_ontology};
pub use entity::gazetteer_entity::*;
pub use entity::grammar_entity::*;
pub use ontology::*;
pub use language::*;

pub static ONTOLOGY_VERSION: &str = "0.7.0";
