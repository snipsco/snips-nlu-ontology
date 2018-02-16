#![allow(non_camel_case_types)]

use std::convert::From;
use std::ffi::CString;
use std::slice;

use libc;

use ffi_utils::*;
use snips_nlu_ontology::*;


#[repr(C)]
#[derive(Debug)]
pub struct CBuiltinEntity {
    pub value: *const libc::c_char,
    pub range_start: libc::c_int,
    pub range_end: libc::c_int,
    pub entity: ::CSlotValue,
    pub entity_kind: CBuiltinEntityKind,
}

impl From<::BuiltinEntity> for CBuiltinEntity {
    fn from(e: ::BuiltinEntity) -> CBuiltinEntity {
        Self {
            value: CString::new(e.value).unwrap().into_raw(), // String can not contains 0
            range_start: e.range.start as libc::c_int,
            range_end: e.range.end as libc::c_int,
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
    pub size: libc::c_int, // Note: we can't use `libc::size_t` because it's not supported by JNA
}

impl CBuiltinEntityArray {
    pub fn from(input: Vec<CBuiltinEntity>) -> Self {
        Self {
            size: input.len() as libc::c_int,
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

#[no_mangle]
pub extern "C" fn nlu_ontology_all_builtin_entities() -> *const CStringArray {
    let arr = Box::new(CStringArray::from(
        BuiltinEntityKind::all()
            .iter()
            .map(|l| l.identifier().to_string())
            .collect::<Vec<String>>()));
    Box::into_raw(arr)
}
