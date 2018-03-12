use std::ffi::{CStr, CString};
use std::slice;
use std::sync::Arc;
use std::str::FromStr;

use failure::ResultExt;
use libc;
use serde_json;

use Result;
use snips_nlu_ontology::{BuiltinEntity, BuiltinEntityKind, Language};
use snips_nlu_ontology_ffi_macros::{CBuiltinEntity, CBuiltinEntityArray, CStringArray};
use snips_nlu_ontology_parsers::BuiltinEntityParser;

#[repr(C)]
pub struct CBuiltinEntityParser {
    pub parser: *const libc::c_void,
}

macro_rules! get_parser {
    ($opaque:ident) => {{
        let container: &$crate::CBuiltinEntityParser = unsafe { &*$opaque };
        let x = container.parser as *const ::snips_nlu_ontology_parsers::BuiltinEntityParser;
        unsafe { &*x }
    }};
}

macro_rules! get_parser_mut {
    ($opaque:ident) => {{
        let container: &$crate::CBuiltinEntityParser = unsafe { &*$opaque };
        let x = container.parser as *mut ::snips_nlu_ontology_parsers::BuiltinEntityParser;
        unsafe { &mut *x }
    }};
}

pub fn create_builtin_entity_parser(
    ptr: *mut *const CBuiltinEntityParser,
    lang: *const libc::c_char,
) -> Result<()> {
    let lang = unsafe { CStr::from_ptr(lang) }.to_str()?;
    let lang = Language::from_str(&*lang.to_uppercase())?;
    let parser = BuiltinEntityParser::get(lang);

    unsafe {
        let container = CBuiltinEntityParser {
            parser: Arc::into_raw(parser) as *const libc::c_void,
        };
        *ptr = Box::into_raw(Box::new(container))
    }
    Ok(())
}

pub fn extract_entity_c(
    ptr: *const CBuiltinEntityParser,
    sentence: *const libc::c_char,
    filter_entity_kinds: *const CStringArray,
    results: *mut *const CBuiltinEntityArray,
) -> Result<()> {
    let c_entities = extract_entity(ptr, sentence, filter_entity_kinds)?
        .into_iter()
        .map(CBuiltinEntity::from)
        .collect::<Vec<CBuiltinEntity>>();
    let c_entities = Box::new(CBuiltinEntityArray::from(c_entities));

    unsafe {
        *results = Box::into_raw(c_entities);
    }

    Ok(())
}

pub fn extract_entity_json(
    ptr: *const CBuiltinEntityParser,
    sentence: *const libc::c_char,
    filter_entity_kinds: *const CStringArray,
    results: *mut *const libc::c_char,
) -> Result<()> {
    let entities = extract_entity(ptr, sentence, filter_entity_kinds)?;
    let json = serde_json::to_string(&entities)?;

    let cs = CString::new(json).unwrap(); // String can not contains 0
    unsafe { *results = cs.into_raw() }

    Ok(())
}

pub fn extract_entity(
    ptr: *const CBuiltinEntityParser,
    sentence: *const libc::c_char,
    filter_entity_kinds: *const CStringArray,
) -> Result<Vec<BuiltinEntity>> {
    let parser = get_parser!(ptr);
    let sentence = unsafe { CStr::from_ptr(sentence) }.to_str()?;

    let opt_filters: Option<Vec<_>> = if !filter_entity_kinds.is_null() {
        let filters = unsafe {
            let array = &*filter_entity_kinds;
            slice::from_raw_parts(array.data, array.size as usize)
        }.into_iter()
            .map(|&ptr| {
                Ok(unsafe { CStr::from_ptr(ptr) }
                    .to_str()
                    .map_err(::failure::Error::from)
                    .and_then(|s| {
                        BuiltinEntityKind::from_identifier(s)
                            .with_context(|_| format!("`{}` isn't a known builtin entity kind", s))
                            .map_err(::failure::Error::from)
                    })?)
            })
            .collect::<Result<Vec<_>>>()?;
        Some(filters)
    } else {
        None
    };
    let opt_filters = opt_filters.as_ref().map(|vec| vec.as_slice());

    Ok(parser.extract_entities(sentence, opt_filters))
}

pub fn destroy_builtin_entity_parser(ptr: *mut CBuiltinEntityParser) -> Result<()> {
    let parser = get_parser_mut!(ptr);
    unsafe {
        let _ = Arc::from_raw(parser);
    }
    Ok(())
}