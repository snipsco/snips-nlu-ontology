use std::ffi::CString;
use libc;

use snips_nlu_ontology::{Language, RustlingLang};
use ffi_utils::CStringArray;

// We are forced to wrap this Box because lazy_static! require to be Sync but
// ffi's type `*const libc::c_char` isn't
struct DummyWrapper(Box<[*const libc::c_char]>);

unsafe impl Sync for DummyWrapper {}

#[no_mangle]
pub extern "C" fn nlu_ontology_supported_languages() -> CStringArray {
    lazy_static! {
        static ref ALL: DummyWrapper = {
            DummyWrapper(
                Language::all()
                    .iter()
                    .map(|l| l.to_string().to_lowercase())
                    .map(|l| CString::new(l).unwrap().into_raw() as *const libc::c_char)
                    .collect::<Vec<*const libc::c_char>>()
                    .into_boxed_slice()
            )
        };
    }

    CStringArray {
        data: ALL.0.as_ptr() as *const *const libc::c_char,
        size: ALL.0.len() as libc::int32_t,
    }
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
