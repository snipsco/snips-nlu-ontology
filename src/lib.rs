#[macro_use]
extern crate serde_derive;

pub mod errors;
pub mod language;
pub mod macros;

pub mod entity;
mod ontology;

pub use entity::builtin_entity::{
    complete_entity_ontology, language_entity_ontology, BuiltinEntity, BuiltinEntityKind,
    IntoBuiltinEntityKind,
};
pub use entity::gazetteer_entity::*;
pub use entity::grammar_entity::*;
pub use language::*;
pub use ontology::*;

pub static ONTOLOGY_VERSION: &str = "0.7.0";
