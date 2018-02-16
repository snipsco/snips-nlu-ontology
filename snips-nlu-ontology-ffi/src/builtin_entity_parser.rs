#![allow(non_camel_case_types)]

use std::ffi::{CStr, CString};
use std::sync::{Arc, Mutex};
use std::slice;

use libc;

use errors::*;
use snips_nlu_ontology::*;

lazy_static! {
    static ref LAST_ERROR: Mutex<String> = Mutex::new("".to_string());
}

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

#[repr(C)]
pub enum CResult {
    RESULT_OK = 0,
    RESULT_KO = 1,
}

macro_rules! wrap {
    ($e:expr) => { match $e {
        Ok(_) => { CResult::RESULT_OK }
        Err(e) => {
            use error_chain::ChainedError;
            let msg = e.display_chain().to_string();
            eprintln!("{}", msg);
            match LAST_ERROR.lock() {
                Ok(mut guard) => *guard = msg,
                Err(_) => () /* curl up and cry */
            }
            CResult::RESULT_KO
        }
    }}
}

#[repr(C)]
#[derive(Debug)]
pub struct CBuiltinEntity {
    pub value: *const libc::c_char,
    pub range_start: libc::c_int,
    pub range_end: libc::c_int,
    pub entity: ::CSlotValue,
    pub entity_kind: CBuiltinEntityKind,
}

impl CBuiltinEntity {
    fn from(e: ::BuiltinEntity) -> OntologyResult<Self> {
        Ok(Self {
            value: CString::new(e.value)?.into_raw(),
            range_start: e.range.start as libc::c_int,
            range_end: e.range.end as libc::c_int,
            entity: ::CSlotValue::from(e.entity)?,
            entity_kind: e.entity_kind.into(),
        })
    }
}

impl Drop for CBuiltinEntity {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.value as *mut libc::c_char) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum CBuiltinEntityKind {
    KIND_AMOUNT_OF_MONEY,
    KIND_DURATION,
    KIND_NUMBER,
    KIND_ORDINAL,
    KIND_TEMPERATURE,
    KIND_TIME,
    KIND_PERCENTAGE,
}

impl From<CBuiltinEntityKind> for BuiltinEntityKind {
    fn from(c_value: CBuiltinEntityKind) -> Self {
        BuiltinEntityKind::from(&c_value)
    }
}

impl<'a> From<&'a CBuiltinEntityKind> for BuiltinEntityKind {
    fn from(c_value: &CBuiltinEntityKind) -> Self {
        match *c_value {
            CBuiltinEntityKind::KIND_AMOUNT_OF_MONEY => BuiltinEntityKind::AmountOfMoney,
            CBuiltinEntityKind::KIND_DURATION => BuiltinEntityKind::Duration,
            CBuiltinEntityKind::KIND_NUMBER => BuiltinEntityKind::Number,
            CBuiltinEntityKind::KIND_ORDINAL => BuiltinEntityKind::Ordinal,
            CBuiltinEntityKind::KIND_TEMPERATURE => BuiltinEntityKind::Temperature,
            CBuiltinEntityKind::KIND_TIME => BuiltinEntityKind::Time,
            CBuiltinEntityKind::KIND_PERCENTAGE => BuiltinEntityKind::Percentage,
        }
    }
}

impl From<BuiltinEntityKind> for CBuiltinEntityKind {
    fn from(c_value: BuiltinEntityKind) -> Self {
        CBuiltinEntityKind::from(&c_value)
    }
}

impl<'a> From<&'a BuiltinEntityKind> for CBuiltinEntityKind {
    fn from(c_value: &BuiltinEntityKind) -> Self {
        match *c_value {
            BuiltinEntityKind::AmountOfMoney => CBuiltinEntityKind::KIND_AMOUNT_OF_MONEY,
            BuiltinEntityKind::Duration => CBuiltinEntityKind::KIND_DURATION,
            BuiltinEntityKind::Number => CBuiltinEntityKind::KIND_NUMBER,
            BuiltinEntityKind::Ordinal => CBuiltinEntityKind::KIND_ORDINAL,
            BuiltinEntityKind::Temperature => CBuiltinEntityKind::KIND_TEMPERATURE,
            BuiltinEntityKind::Time => CBuiltinEntityKind::KIND_TIME,
            BuiltinEntityKind::Percentage => CBuiltinEntityKind::KIND_PERCENTAGE,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CRustlingEntityArray {
    pub data: *const CBuiltinEntity,
    pub size: libc::c_int, // Note: we can't use `libc::size_t` because it's not supported by JNA
}

impl CRustlingEntityArray {
    pub fn from(input: Vec<CBuiltinEntity>) -> OntologyResult<Self> {
        Ok(Self {
            size: input.len() as libc::c_int,
            data: Box::into_raw(input.into_boxed_slice()) as *const CBuiltinEntity,
        })
    }
}

impl Drop for CRustlingEntityArray {
    fn drop(&mut self) {
        let _ = unsafe {
            Box::from_raw(slice::from_raw_parts_mut(
                    self.data as *mut CRustlingEntityArray,
                    self.size as usize))
        };
    }
}

#[no_mangle]
pub extern "C" fn nlu_ontology_create_rustling_parser(
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
    results: *mut *const CRustlingEntityArray,
) -> CResult {
    wrap!(extract_entity(ptr, sentence, filter_entity_kinds, filter_entity_kinds_size, results))
}

#[no_mangle]
pub extern "C" fn nlu_ontology_destroy_rustling_parser(
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
    results: *mut *const CRustlingEntityArray,
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
        .collect::<OntologyResult<_>>()?;
    let c_entities = Box::new(CRustlingEntityArray::from(c_entities)?);

    unsafe {
        *results = Box::into_raw(c_entities);
    }

    Ok(())
}

