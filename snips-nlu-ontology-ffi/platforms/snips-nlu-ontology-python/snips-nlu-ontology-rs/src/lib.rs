extern crate libc;
extern crate snips_nlu_ontology_ffi;

use snips_nlu_ontology_ffi::{CResult, CStringArray, CBuiltinEntityParser, CBuiltinEntityArray};

#[doc(hidden)]
#[macro_export]
macro_rules! export_c_symbol {
    ($alias:ident, fn $name:ident($( $arg:ident : $type:ty ),*) -> $ret:ty) => {
        #[no_mangle]
        pub extern "C" fn $alias($( $arg : $type),*) -> $ret {
            ::snips_nlu_ontology_ffi::$name($( $arg ),*)
        }
    };
    ($alias:ident, fn $name:ident($( $arg:ident : $type:ty ),*)) => {
        export_c_symbol!($alias, fn $name($( $arg : $type),*) -> ());
    }
}

/// Workaround for rust-lang/rust#6342

export_c_symbol!(ffi_nlu_ontology_version, fn nlu_ontology_version() -> *const ::libc::c_char);
export_c_symbol!(ffi_nlu_ontology_supported_languages, fn nlu_ontology_supported_languages() -> CStringArray);
export_c_symbol!(ffi_nlu_ontology_all_builtin_entities, fn nlu_ontology_all_builtin_entities() -> CStringArray);
export_c_symbol!(ffi_nlu_ontology_supported_builtin_entities, fn nlu_ontology_supported_builtin_entities(language: *const ::libc::c_char,results: *mut *const CStringArray) -> CResult);
export_c_symbol!(ffi_nlu_ontology_create_builtin_entity_parser, fn nlu_ontology_create_builtin_entity_parser(ptr: *mut *const CBuiltinEntityParser,lang: *const ::libc::c_char) -> CResult);
export_c_symbol!(ffi_nlu_ontology_extract_entities, fn nlu_ontology_extract_entities(ptr: *const CBuiltinEntityParser,sentence: *const ::libc::c_char,filter_entity_kinds: *const CStringArray,results: *mut *const CBuiltinEntityArray) -> CResult);
export_c_symbol!(ffi_nlu_ontology_extract_entities_json, fn nlu_ontology_extract_entities_json(ptr: *const CBuiltinEntityParser,sentence: *const ::libc::c_char,filter_entity_kinds: *const CStringArray,results: *mut *const ::libc::c_char) -> CResult);
export_c_symbol!(ffi_nlu_ontology_destroy_builtin_entity_array, fn nlu_ontology_destroy_builtin_entity_array(ptr: *mut CBuiltinEntityArray) -> CResult);
export_c_symbol!(ffi_nlu_ontology_destroy_builtin_entity_parser, fn nlu_ontology_destroy_builtin_entity_parser(ptr: *mut CBuiltinEntityParser) -> CResult);
export_c_symbol!(ffi_nlu_ontology_get_last_error, fn nlu_ontology_get_last_error(error: *mut *const ::libc::c_char) -> CResult);
export_c_symbol!(ffi_nlu_ontology_destroy_string_array, fn nlu_ontology_destroy_string_array(ptr: *mut CStringArray) -> CResult);
export_c_symbol!(ffi_nlu_ontology_destroy_string, fn nlu_ontology_destroy_string(ptr: *mut ::libc::c_char) -> CResult);
