use std::ffi::{CStr, CString};
use std::sync::{Arc, Mutex};
use std::slice;

use libc;

use errors::*;
use snips_nlu_ontology::rustling_parser::*;
use snips_nlu_ontology::BuiltinEntityKind;

lazy_static! {
    static ref LAST_ERROR: Mutex<String> = Mutex::new("".to_string());
}

#[repr(C)]
pub enum CResult {
    OK, KO
}

macro_rules! wrap {
    ($e:expr) => { match $e {
        Ok(_) => { CResult::OK }
        Err(e) => {
            use error_chain::ChainedError;
            let msg = e.display_chain().to_string();
            eprintln!("{}", msg);
            match LAST_ERROR.lock() {
                Ok(mut guard) => *guard = msg,
                Err(_) => () /* curl up and cry */
            }
            CResult::KO
        }
    }}
}

#[repr(C)]
#[derive(Debug)]
pub struct CRustlingEntity {
    pub value: *const libc::c_char,
    pub range_start: libc::c_int,
    pub range_end: libc::c_int,
    pub entity: ::CSlotValue,
    pub entity_kind: ::CBuiltinEntityKind,
}

impl CRustlingEntity {
    fn from(e: ::RustlingEntity) -> OntologyResult<Self> {
        Ok(Self {
            value: CString::new(e.value)?.into_raw(),
            range_start: e.range.start as libc::c_int,
            range_end: e.range.end as libc::c_int,
            entity: ::CSlotValue::from(e.entity)?,
            entity_kind: e.entity_kind.into(),
        })
    }
}

impl Drop for CRustlingEntity {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.value as *mut libc::c_char) };
    }
}

#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum CBuiltinEntityKind {
    AMOUNT_OF_MONEY,
    DURATION,
    NUMBER,
    ORDINAL,
    TEMPERATURE,
    TIME,
    PERCENTAGE,
}

impl From<CBuiltinEntityKind> for BuiltinEntityKind {
    fn from(c_value: CBuiltinEntityKind) -> Self {
        BuiltinEntityKind::from(&c_value)
    }
}

impl<'a> From<&'a CBuiltinEntityKind> for BuiltinEntityKind {
    fn from(c_value: &CBuiltinEntityKind) -> Self {
        match *c_value {
            CBuiltinEntityKind::AMOUNT_OF_MONEY => BuiltinEntityKind::AmountOfMoney,
            CBuiltinEntityKind::DURATION => BuiltinEntityKind::Duration,
            CBuiltinEntityKind::NUMBER => BuiltinEntityKind::Number,
            CBuiltinEntityKind::ORDINAL => BuiltinEntityKind::Ordinal,
            CBuiltinEntityKind::TEMPERATURE => BuiltinEntityKind::Temperature,
            CBuiltinEntityKind::TIME => BuiltinEntityKind::Time,
            CBuiltinEntityKind::PERCENTAGE => BuiltinEntityKind::Percentage,
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
            BuiltinEntityKind::AmountOfMoney => CBuiltinEntityKind::AMOUNT_OF_MONEY,
            BuiltinEntityKind::Duration => CBuiltinEntityKind::DURATION,
            BuiltinEntityKind::Number => CBuiltinEntityKind::NUMBER,
            BuiltinEntityKind::Ordinal => CBuiltinEntityKind::ORDINAL,
            BuiltinEntityKind::Temperature => CBuiltinEntityKind::TEMPERATURE,
            BuiltinEntityKind::Time => CBuiltinEntityKind::TIME,
            BuiltinEntityKind::Percentage => CBuiltinEntityKind::PERCENTAGE,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CRustlingEntityArray {
    pub data: *const CRustlingEntity,
    pub size: libc::c_int, // Note: we can't use `libc::size_t` because it's not supported by JNA
}

impl CRustlingEntityArray {
    pub fn from(input: Vec<::CRustlingEntity>) -> OntologyResult<Self> {
        Ok(Self {
            size: input.len() as libc::c_int,
            data: Box::into_raw(input.into_boxed_slice()) as *const CRustlingEntity,
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
    ptr: *mut *const RustlingParser,
) -> CResult {
    let parser = RustlingParser::get(lang.into());
    unsafe {
        *ptr = Arc::into_raw(parser);
    }
    CResult::OK
}

#[no_mangle]
pub extern "C" fn nlu_ontology_extract_entities(
    ptr: *const RustlingParser,
    sentence: *const libc::c_char,
    filter_entity_kinds: *const CBuiltinEntityKind,
    filter_entity_kinds_size: usize,
    results: *mut *const CRustlingEntityArray,
) -> CResult {
    wrap!(extract_entity(ptr, sentence, filter_entity_kinds, filter_entity_kinds_size, results))
}

#[no_mangle]
pub extern "C" fn nlu_ontology_destroy_rustling_parser(
    ptr: *mut RustlingParser,
) -> CResult {
    unsafe {
        let _ = Arc::from_raw(ptr);
    }
    CResult::OK
}

fn extract_entity(
    ptr: *const RustlingParser,
    sentence: *const libc::c_char,
    filter_entity_kinds: *const CBuiltinEntityKind,
    filter_entity_kinds_size: usize,
    results: *mut *const CRustlingEntityArray,
) -> OntologyResult<()> {
    let parser = unsafe { &*ptr };
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
        .map(CRustlingEntity::from)
        .collect::<OntologyResult<_>>()?;
    let c_entities = Box::new(CRustlingEntityArray::from(c_entities)?);

    unsafe {
        *results = Box::into_raw(c_entities);
    }

    Ok(())
}

