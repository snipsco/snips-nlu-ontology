use std::ffi::CStr;
use std::slice;

use libc;
use serde_json;

use Result;
use ffi_utils::{CStringArray, CReprOf, RawPointerConverter};
use snips_nlu_ontology_parsers::{GazetteerEntityMatch, GazetteerParser, GazetteerParserBuilder};

#[repr(C)]
pub struct CGazetteerEntityParser(*const libc::c_void);

macro_rules! get_parser {
    ($opaque:ident) => {{
        let container: &$crate::CGazetteerEntityParser = unsafe { &*$opaque };
        let x = container.0 as *const ::snips_nlu_ontology_parsers::GazetteerParser<String>;
        unsafe { &*x }
    }};
}

pub fn load_gazetteer_entity_parser(
    ptr: *mut *const CGazetteerEntityParser,
    path: *const libc::c_char,
) -> Result<()> {
    let parser_path = unsafe { CStr::from_ptr(path) }.to_str()?;
    let gazetteer_parser = GazetteerParser::<String>::from_path(parser_path)?;
    let c_parser = CGazetteerEntityParser(gazetteer_parser.into_raw_pointer() as _).into_raw_pointer();

    unsafe {
        *ptr = c_parser;
    }
    Ok(())
}

pub fn build_gazetteer_entity_parser(
    ptr: *mut *const CGazetteerEntityParser,
    json_config: *const libc::c_char,
) -> Result<()> {
    let json_config = unsafe { CStr::from_ptr(json_config) }.to_str()?;
    let gazetteer_parser = serde_json::from_str::<GazetteerParserBuilder>(json_config)?
        .build::<String>()?;
    let c_parser = CGazetteerEntityParser(gazetteer_parser.into_raw_pointer() as _).into_raw_pointer();

    unsafe {
        *ptr = c_parser;
    }
    Ok(())
}

pub fn persist_gazetteer_entity_parser(
    ptr: *const CGazetteerEntityParser,
    path: *const libc::c_char
) -> Result<()> {
    let parser = get_parser!(ptr);
    let parser_path = unsafe { CStr::from_ptr(path) }.to_str()?;
    parser.persist(parser_path)?;
    Ok(())
}

pub fn extract_gazetteer_entity_json(
    ptr: *const CGazetteerEntityParser,
    sentence: *const libc::c_char,
    filter_entity_kinds: *const CStringArray,
    results: *mut *const libc::c_char,
) -> Result<()> {
    let entities = extract_gazetteer_entity(ptr, sentence, filter_entity_kinds)?;
    let json = ::serde_json::to_string(&entities)?;

    let cs = convert_to_c_string!(json);
    unsafe { *results = cs }

    Ok(())
}

pub fn extract_gazetteer_entity(
    ptr: *const CGazetteerEntityParser,
    sentence: *const libc::c_char,
    filter_entity_kinds: *const CStringArray,
) -> Result<Vec<GazetteerEntityMatch<String>>> {
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
                    .map_err(::failure::Error::from)?
                    .to_string())
            })
            .collect::<Result<Vec<_>>>()?;
        Some(filters)
    } else {
        None
    };

    parser.extract_entities(sentence, opt_filters.as_ref())
}

pub fn destroy_gazetteer_entity_parser(ptr: *mut CGazetteerEntityParser) -> Result<()> {
    unsafe {
        let parser = CGazetteerEntityParser::from_raw_pointer(ptr)?.0;
        let _ = GazetteerParser::<String>::from_raw_pointer(parser as _);
    }
    Ok(())
}
