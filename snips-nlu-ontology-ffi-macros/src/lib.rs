#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate snips_nlu_ontology;

type Result<T> = ::std::result::Result<T, ::failure::Error>;

#[macro_use]
pub mod ffi_utils;
pub mod failure_ext;

mod builtin_entity;
mod ontology;
mod language;

use snips_nlu_ontology::*;

pub use ontology::*;
pub use language::*;
pub use builtin_entity::*;
pub use ffi_utils::*;

#[macro_export]
macro_rules! export_nlu_ontology_c_symbols {
    () => {
        #[no_mangle]
        pub extern "C" fn nlu_ontology_version() -> *const libc::c_char {
            ::std::ffi::CString::new(snips_nlu_ontology::ONTOLOGY_VERSION).unwrap().into_raw()
        }

        #[no_mangle]
        pub extern "C" fn nlu_ontology_get_last_error(error: *mut *const libc::c_char) -> $crate::CResult {
            wrap!($crate::get_last_error(error))
        }

        #[no_mangle]
        pub extern "C" fn nlu_ontology_destroy_string_array(ptr: *mut $crate::CStringArray) -> $crate::CResult {
            wrap!($crate::destroy(ptr))
        }

        #[no_mangle]
        pub extern "C" fn nlu_ontology_destroy_string(ptr: *mut libc::c_char) -> $crate::CResult {
            wrap!($crate::destroy_string(ptr))
        }

        #[no_mangle]
        pub extern "C" fn nlu_ontology_supported_languages() -> $crate::CStringArray {
            $crate::supported_languages()
        }

        #[no_mangle]
        pub extern "C" fn nlu_ontology_all_builtin_entities() -> $crate::CStringArray {
            $crate::all_builtin_entities()
        }

        #[no_mangle]
        pub extern "C" fn nlu_ontology_supported_builtin_entities(
            language: *const libc::c_char,
            results: *mut *const $crate::CStringArray,
        ) -> $crate::CResult {
            wrap!($crate::get_supported_builtin_entities(language, results))
        }
    };
}

