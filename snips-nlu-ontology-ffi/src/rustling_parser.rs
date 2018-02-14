use std::ffi::{CStr, CString};
use std::sync::Arc;
use libc;

use errors::*;
use snips_nlu_ontology::rustling_parser::*;
use snips_nlu_ontology::BuiltinEntityKind;

#[repr(C)]
pub enum CResult {
    OK, KO
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
    filter_entity_kinds: *const Vec<CBuiltinEntityKind>,
    results: *mut Box<Vec<CRustlingEntity>>,
) -> CResult {
    let parser = unsafe { &*ptr };
    let sentence = unsafe { CStr::from_ptr(sentence) }.to_str().unwrap();

    let opt_filter: Option<Vec<_>> = unsafe { filter_entity_kinds.as_ref() }
        .map(|vec| vec.into_iter().map(BuiltinEntityKind::from).collect());
    let opt_filter = opt_filter.as_ref().map(|vec| vec.as_slice());

    let c_entities = parser.extract_entities(sentence, opt_filter)
        .into_iter()
        .map(CRustlingEntity::from)
        .collect::<OntologyResult<_>>()
        .unwrap();
    let c_entities = Box::new(c_entities);

    unsafe {
        *results = c_entities;
    }

    CResult::OK
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
