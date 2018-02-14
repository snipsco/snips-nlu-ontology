#[macro_use] extern crate error_chain;
extern crate itertools;
#[macro_use] extern crate lazy_static;
extern crate rustling_ontology;
#[macro_use] extern crate serde_derive;
#[cfg(test)]
extern crate serde_json;

pub mod errors;
pub mod builtin_entities;

mod ontology;

pub use rustling_ontology::Lang as RustlingLang;
pub use builtin_entities::*;
pub use ontology::*;

pub enum Language {
    DE, EN, ES, FR, KO, ZH
}

impl From<Language> for RustlingLang {
    fn from(lang: Language) -> Self {
        match lang {
            Language::DE => RustlingLang::DE,
            Language::EN => RustlingLang::EN,
            Language::ES => RustlingLang::ES,
            Language::FR => RustlingLang::FR,
            Language::KO => RustlingLang::KO,
            Language::ZH => RustlingLang::ZH,
        }
    }
}
