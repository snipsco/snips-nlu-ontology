use std::ffi::CStr;
use std::sync::Arc;
use std::slice;

use libc;

use errors::*;
use snips_nlu_ontology::*;
use builtin_entity::*;
use ffi_utils::*;

#[repr(C)]
pub struct CBuiltinEntityParser {
    pub parser: *const libc::c_void,
}

macro_rules! get_parser {
    ($opaque:ident) => {{
        let container: &CBuiltinEntityParser = unsafe { &*$opaque };
        let x = container.parser as *const ::BuiltinEntityParser;
        unsafe { &*x }
    }};
}

macro_rules! get_parser_mut {
    ($opaque:ident) => {{
        let container: &CBuiltinEntityParser = unsafe { &*$opaque };
        let x = container.parser as *mut ::BuiltinEntityParser;
        unsafe { &mut *x }
    }};
}


#[no_mangle]
pub extern "C" fn nlu_ontology_create_builtin_entity_parser(
    lang: ::CLanguage,
    ptr: *mut *const CBuiltinEntityParser,
) -> CResult {
    wrap!(create_builtin_entity_parser(lang, ptr))
}

#[no_mangle]
pub extern "C" fn nlu_ontology_extract_entities(
    ptr: *const CBuiltinEntityParser,
    sentence: *const libc::c_char,
    filter_entity_kinds: *const CBuiltinEntityKind,
    filter_entity_kinds_size: usize,
    results: *mut *const CBuiltinEntityArray,
) -> CResult {
    wrap!(extract_entity(ptr, sentence, filter_entity_kinds, filter_entity_kinds_size, results))
}

#[no_mangle]
pub extern "C" fn nlu_ontology_destroy_builtin_entity_parser(
    ptr: *mut CBuiltinEntityParser,
) -> CResult {
    let parser = get_parser_mut!(ptr);
    unsafe {
        let _ = Arc::from_raw(parser);
    }
    CResult::RESULT_OK
}

fn create_builtin_entity_parser(
    lang: ::CLanguage,
    ptr: *mut *const CBuiltinEntityParser,
) -> OntologyResult<()> {
    let parser = BuiltinEntityParser::get(lang.into());

    unsafe {
        let container = CBuiltinEntityParser {
            parser: Arc::into_raw(parser) as *const libc::c_void,
        };
        *ptr = Box::into_raw(Box::new(container))
    }
    Ok(())
}

fn extract_entity(
    ptr: *const CBuiltinEntityParser,
    sentence: *const libc::c_char,
    filter_entity_kinds: *const CBuiltinEntityKind,
    filter_entity_kinds_size: usize,
    results: *mut *const CBuiltinEntityArray,
) -> OntologyResult<()> {
    let parser = get_parser!(ptr);
    let sentence = unsafe { CStr::from_ptr(sentence) }.to_str()?;

    let opt_filter: Option<Vec<_>> = unsafe { filter_entity_kinds.as_ref() }
        .map(|vec| {
            unsafe { slice::from_raw_parts(vec, filter_entity_kinds_size) }
                .into_iter()
                .map(BuiltinEntityKind::from)
                .collect()
        });
    let opt_filter = opt_filter.as_ref().map(|vec| vec.as_slice());

    let c_entities = parser.extract_entities(sentence, opt_filter)
        .into_iter()
        .map(CBuiltinEntity::from)
        .collect::<Vec<CBuiltinEntity>>();
    let c_entities = Box::new(CBuiltinEntityArray::from(c_entities));

    unsafe {
        *results = Box::into_raw(c_entities);
    }

    Ok(())
}

