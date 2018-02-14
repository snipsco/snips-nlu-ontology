extern crate snips_nlu_ontology;
#[macro_use] extern crate error_chain;
extern crate libc;

pub mod errors;

mod rustling_parser;
mod ontology;

use snips_nlu_ontology::*;

pub use ontology::*;
pub use rustling_parser::*;

#[repr(C)]
pub enum CLanguage {
    DE, EN, ES, FR, KO, ZH
}

impl From<CLanguage> for Language {
    fn from(c_lang: CLanguage) -> Self {
        match c_lang {
            CLanguage::DE => Language::DE,
            CLanguage::EN => Language::EN,
            CLanguage::ES => Language::ES,
            CLanguage::FR => Language::FR,
            CLanguage::KO => Language::KO,
            CLanguage::ZH => Language::ZH,
        }
    }
}

impl From<CLanguage> for RustlingLang {
    fn from(c_lang: CLanguage) -> Self {
        Language::from(c_lang).into()
    }
}
