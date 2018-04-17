extern crate failure;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate snips_nlu_ontology;
extern crate snips_nlu_ontology_ffi_macros;
extern crate snips_nlu_ontology_parsers;

type Result<T> = ::std::result::Result<T, ::failure::Error>;

mod builtin_entity_parser;

pub use builtin_entity_parser::*;

#[macro_export]
macro_rules! export_nlu_ontology_parsers_c_symbols {
    () => {
        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_create_builtin_entity_parser(
            ptr: *mut *const $crate::CBuiltinEntityParser,
            lang: *const libc::c_char,
        ) -> snips_nlu_ontology_ffi_macros::CResult {
            wrap!($crate::create_builtin_entity_parser(ptr, lang))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_extract_entities(
            ptr: *const $crate::CBuiltinEntityParser,
            sentence: *const libc::c_char,
            filter_entity_kinds: *const snips_nlu_ontology_ffi_macros::CStringArray,
            results: *mut *const snips_nlu_ontology_ffi_macros::CBuiltinEntityArray,
        ) -> snips_nlu_ontology_ffi_macros::CResult {
            wrap!($crate::extract_entity_c(
                ptr,
                sentence,
                filter_entity_kinds,
                results
            ))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_extract_entities_json(
            ptr: *const $crate::CBuiltinEntityParser,
            sentence: *const libc::c_char,
            filter_entity_kinds: *const snips_nlu_ontology_ffi_macros::CStringArray,
            results: *mut *const libc::c_char,
        ) -> snips_nlu_ontology_ffi_macros::CResult {
            wrap!($crate::extract_entity_json(
                ptr,
                sentence,
                filter_entity_kinds,
                results
            ))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_destroy_builtin_entity_array(
            ptr: *mut snips_nlu_ontology_ffi_macros::CBuiltinEntityArray,
        ) -> snips_nlu_ontology_ffi_macros::CResult {
            wrap!(snips_nlu_ontology_ffi_macros::destroy(ptr))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_destroy_builtin_entity_parser(
            ptr: *mut $crate::CBuiltinEntityParser,
        ) -> snips_nlu_ontology_ffi_macros::CResult {
            wrap!($crate::destroy_builtin_entity_parser(ptr))
        }
    }
}
