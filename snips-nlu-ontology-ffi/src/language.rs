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
pub enum CLanguage {
    DE,
    EN,
    ES,
    FR,
    KO,
    ZH,
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
