#![allow(non_camel_case_types)]

use std::convert::From;
use std::ffi::{CStr, CString};
use std::slice;
use std::str::FromStr;

use libc;

use errors::*;
use ffi_utils::*;
use snips_nlu_ontology::*;


#[repr(C)]
#[derive(Debug)]
pub struct CBuiltinEntity {
    pub entity: ::CSlotValue,
    pub value: *const libc::c_char,
    pub entity_kind: CBuiltinEntityKind,
    pub range_start: libc::int32_t,
    pub range_end: libc::int32_t,
}

impl From<::BuiltinEntity> for CBuiltinEntity {
    fn from(e: ::BuiltinEntity) -> CBuiltinEntity {
        Self {
            value: CString::new(e.value).unwrap().into_raw(), // String can not contains 0
            range_start: e.range.start as libc::int32_t,
            range_end: e.range.end as libc::int32_t,
            entity: ::CSlotValue::from(e.entity),
            entity_kind: e.entity_kind.into(),
        }
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
pub struct CBuiltinEntityArray {
    pub data: *const CBuiltinEntity,
    pub size: libc::int32_t, // Note: we can't use `libc::size_t` because it's not supported by JNA
}

impl CBuiltinEntityArray {
    pub fn from(input: Vec<CBuiltinEntity>) -> Self {
        Self {
            size: input.len() as libc::int32_t,
            data: Box::into_raw(input.into_boxed_slice()) as *const CBuiltinEntity,
        }
    }
}

impl Drop for CBuiltinEntityArray {
    fn drop(&mut self) {
        let _ = unsafe {
            Box::from_raw(slice::from_raw_parts_mut(
                self.data as *mut CBuiltinEntityArray,
                self.size as usize))
        };
    }
}

// We are forced to wrap this Box because lazy_static! require to be Sync but
// ffi's type `*const libc::c_char` isn't
struct DummyWrapper(Box<[*const libc::c_char]>);

unsafe impl Sync for DummyWrapper {}

#[no_mangle]
pub extern "C" fn nlu_ontology_all_builtin_entities() -> CStringArray {
    lazy_static! {
        static ref ALL: DummyWrapper = {
            DummyWrapper(
                BuiltinEntityKind::all()
                    .iter()
                    .map(|l| l.identifier().to_string())
                    .map(|l| CString::new(l).unwrap().into_raw() as *const libc::c_char)
                    .collect::<Vec<*const libc::c_char>>()
                    .into_boxed_slice()
            )
        };
    }

    CStringArray {
        data: ALL.0.as_ptr() as *const *const libc::c_char,
        size: ALL.0.len() as libc::int32_t,
    }
}

#[no_mangle]
pub extern "C" fn nlu_ontology_supported_builtin_entities(
    language: *const libc::c_char,
    results: *mut *const CStringArray) -> CResult {
    wrap!(get_supported_builtin_entities(language, results))
}

fn get_supported_builtin_entities(
    language: *const libc::c_char,
    results: *mut *const CStringArray) -> OntologyResult<()> {
    let language_str = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(&*language_str.to_uppercase())?;
    let entities = BuiltinEntityKind::all().iter()
        .filter(|e| e.supported_languages().contains(&language))
        .map(|e| e.identifier().to_string())
        .collect::<Vec<String>>();
    let c_entities = CStringArray::from(entities);
    unsafe {
        *results = Box::into_raw(Box::new(c_entities));
    }
    Ok(())
}
