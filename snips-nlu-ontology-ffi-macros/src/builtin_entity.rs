#![allow(non_camel_case_types)]

use std::convert::From;
use std::ffi::{CStr, CString};
use std::slice;
use std::str::FromStr;

use libc;

use errors::*;
use ffi_utils::{CStringArray, CReprOf, RawPointerConverter};
use snips_nlu_ontology::{BuiltinEntityKind, Language};

#[repr(C)]
#[derive(Debug)]
pub struct CBuiltinEntity {
    pub entity: ::CSlotValue,
    pub entity_kind: *const libc::c_char,
    pub value: *const libc::c_char,
    pub range_start: libc::int32_t,
    pub range_end: libc::int32_t,
}

impl From<::BuiltinEntity> for CBuiltinEntity {
    fn from(e: ::BuiltinEntity) -> CBuiltinEntity {
        Self {
            entity: ::CSlotValue::from(e.entity),
            entity_kind: CString::new(e.entity_kind.identifier()).unwrap().into_raw(),
            value: CString::new(e.value).unwrap().into_raw(), // String can not contains 0
            range_start: e.range.start as libc::int32_t,
            range_end: e.range.end as libc::int32_t,
        }
    }
}

impl Drop for CBuiltinEntity {
    fn drop(&mut self) {
        take_back_c_string!(self.value);
        take_back_c_string!(self.entity_kind);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CBuiltinEntityArray {
    pub data: *const CBuiltinEntity,
    pub size: libc::int32_t, // Note: we can't use `libc::size_t` because it's not supported by JNA
}

impl From<Vec<CBuiltinEntity>> for CBuiltinEntityArray {
    fn from(input: Vec<CBuiltinEntity>) -> Self {
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
                self.size as usize,
            ))
        };
    }
}

// We are forced to wrap this Box because lazy_static! require to be Sync but
// ffi's type `*const libc::c_char` isn't
struct DummyWrapper(Box<[*const libc::c_char]>);

unsafe impl Sync for DummyWrapper {}

pub fn all_builtin_entities() -> CStringArray {
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

pub fn get_supported_builtin_entities(
    language: *const libc::c_char,
    results: *mut *const CStringArray,
) -> Result<()> {
    let language_str = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(&*language_str.to_uppercase())?;
    let entities = BuiltinEntityKind::all()
        .iter()
        .filter(|e| e.supported_languages().contains(&language))
        .map(|e| e.identifier().to_string())
        .collect::<Vec<_>>();
    let c_entities = CStringArray::c_repr_of(entities)?.into_raw_pointer();
    unsafe {
        *results = c_entities;
    }
    Ok(())
}

pub fn get_builtin_entity_examples(
    builtin_entity_kind: *const libc::c_char,
    language: *const libc::c_char,
    results: *mut *const CStringArray,
) -> Result<()> {
    let entity_kind_str = unsafe { CStr::from_ptr(builtin_entity_kind) }.to_str()?;
    let entity_kind = BuiltinEntityKind::from_identifier(&*entity_kind_str)?;
    let language_str = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(&*language_str.to_uppercase())?;
    let examples = entity_kind.examples(language)
        .into_iter()
        .map(|example| example.to_string())
        .collect::<Vec<_>>();
    let c_examples = CStringArray::c_repr_of(examples)?.into_raw_pointer();
    unsafe {
        *results = c_examples;
    }
    Ok(())
}
