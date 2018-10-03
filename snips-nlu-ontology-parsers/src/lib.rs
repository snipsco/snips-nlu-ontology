#[macro_use]
extern crate failure;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate regex;
extern crate rustling_ontology;
extern crate snips_nlu_ontology as nlu_ontology;
extern crate snips_nlu_utils as nlu_utils;
extern crate gazetteer_entity_parser;
extern crate snips_nlu_ontology;
#[cfg(test)]
extern crate tempfile;

pub mod errors;

mod builtin_entity_parser;
mod conversion;
mod gazetteer_parser;
mod utils;
#[cfg(test)]
mod test_utils;

pub use gazetteer_entity_parser::{EntityValue as GazetteerEntityValue, Parser as GazetteerEntityParser};
pub use builtin_entity_parser::*;
pub use gazetteer_parser::*;
pub use conversion::*;
