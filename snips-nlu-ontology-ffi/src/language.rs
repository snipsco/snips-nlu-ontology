use snips_nlu_ontology::*;
use ffi_utils::*;
use std::convert::From;

#[no_mangle]
pub extern "C" fn nlu_ontology_supported_languages() -> *const CArrayString {
    let arr = Box::new(CArrayString::from(
        Language::all()
            .iter()
            .map(|l| l.to_string().to_lowercase())
            .collect::<Vec<String>>()));
    Box::into_raw(arr)
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum CLanguage {
    LANGUAGE_DE,
    LANGUAGE_EN,
    LANGUAGE_ES,
    LANGUAGE_FR,
    LANGUAGE_KO,
    LANGUAGE_ZH,
}

impl From<CLanguage> for Language {
    fn from(c_lang: CLanguage) -> Self {
        match c_lang {
            CLanguage::LANGUAGE_DE => Language::DE,
            CLanguage::LANGUAGE_EN => Language::EN,
            CLanguage::LANGUAGE_ES => Language::ES,
            CLanguage::LANGUAGE_FR => Language::FR,
            CLanguage::LANGUAGE_KO => Language::KO,
            CLanguage::LANGUAGE_ZH => Language::ZH,
        }
    }
}

impl From<CLanguage> for RustlingLang {
    fn from(c_lang: CLanguage) -> Self {
        Language::from(c_lang).into()
    }
}
