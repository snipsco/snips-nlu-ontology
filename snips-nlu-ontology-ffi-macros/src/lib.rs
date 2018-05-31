#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate snips_nlu_ontology;
#[macro_use]
extern crate ffi_utils;

mod builtin_entity;
mod ontology;
mod language;

use snips_nlu_ontology::*;

pub use builtin_entity::*;
pub use ontology::*;
pub use language::*;

#[macro_export]
macro_rules! export_nlu_ontology_c_symbols {
    () => {
        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_version() -> *const libc::c_char {
            ::std::ffi::CString::new(snips_nlu_ontology::ONTOLOGY_VERSION).unwrap().into_raw()
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_destroy_string_array(ptr: *mut ::ffi_utils::CStringArray) -> ::ffi_utils::SNIPS_RESULT {
            use ffi_utils::RawPointerConverter;
            wrap!(unsafe { ::ffi_utils::CStringArray::from_raw_pointer(ptr) })
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_destroy_string(ptr: *mut libc::c_char) -> ::ffi_utils::SNIPS_RESULT {
            use ffi_utils::RawPointerConverter;
            wrap!(unsafe { ::std::ffi::CString::from_raw_pointer(ptr) })
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_supported_languages() -> ::ffi_utils::CStringArray {
            $crate::supported_languages()
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_all_builtin_entities() -> ::ffi_utils::CStringArray {
            $crate::all_builtin_entities()
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_supported_builtin_entities(
            language: *const libc::c_char,
            results: *mut *const ::ffi_utils::CStringArray,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::get_supported_builtin_entities(language, results))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_builtin_entity_examples(
            builtin_entity_kind: *const libc::c_char,
            language: *const libc::c_char,
            results: *mut *const ::ffi_utils::CStringArray,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::get_builtin_entity_examples(builtin_entity_kind, language, results))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_complete_entity_ontology_json(
            result: *mut *const libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::get_complete_entity_ontology_json(result))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_entity_ontology_json(
            language: *const libc::c_char,
            result: *mut *const libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::get_entity_ontology_json(language, result))
        }
    };
}
