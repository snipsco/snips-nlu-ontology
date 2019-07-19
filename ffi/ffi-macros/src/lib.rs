mod builtin_entity;
mod language;
mod ontology;

use snips_nlu_ontology::*;

pub use builtin_entity::*;
pub use language::*;
pub use ontology::*;

#[macro_export]
macro_rules! export_nlu_ontology_c_symbols {
    () => {
        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_destroy_string_array(
            ptr: *mut ::ffi_utils::CStringArray,
        ) -> ::ffi_utils::SNIPS_RESULT {
            use ffi_utils::RawPointerConverter;
            wrap!(unsafe { ::ffi_utils::CStringArray::from_raw_pointer(ptr) })
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_destroy_string(
            ptr: *mut libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            use ffi_utils::RawPointerConverter;
            wrap!(unsafe { ::std::ffi::CString::from_raw_pointer(ptr) })
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_supported_languages() -> ::ffi_utils::CStringArray {
            $crate::supported_languages()
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_entity_shortname(
            entity_name: *const libc::c_char,
            result: *mut *const libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::get_builtin_entity_shortname(entity_name, result))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_all_builtin_entities() -> ::ffi_utils::CStringArray {
            $crate::all_builtin_entities()
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_all_grammar_entities() -> ::ffi_utils::CStringArray {
            $crate::all_grammar_entities()
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_all_gazetteer_entities() -> ::ffi_utils::CStringArray {
            $crate::all_gazetteer_entities()
        }
    };
}
