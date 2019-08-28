#![allow(non_camel_case_types)]

use crate::errors::*;
use crate::ontology::*;
use ffi_utils::take_back_c_string;
use ffi_utils::{point_to_string, CStringArray, RawPointerConverter};
use lazy_static::lazy_static;
use libc;
use snips_nlu_ontology::{
    BuiltinEntity, BuiltinEntityKind, BuiltinGazetteerEntityKind, GrammarEntityKind,
    IntoBuiltinEntityKind,
};
use std::convert::From;
use std::ffi::{CStr, CString};
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct CBuiltinEntity {
    pub entity: CSlotValue,
    pub entity_kind: *const libc::c_char,
    pub value: *const libc::c_char,
    pub range_start: i32,
    pub range_end: i32,
}

impl From<BuiltinEntity> for CBuiltinEntity {
    fn from(e: BuiltinEntity) -> Self {
        Self {
            entity: CSlotValue::from(e.entity),
            entity_kind: CString::new(e.entity_kind.identifier()).unwrap().into_raw(),
            value: CString::new(e.value).unwrap().into_raw(),
            range_start: e.range.start as i32,
            range_end: e.range.end as i32,
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
    pub size: i32, // Note: we can't use `libc::size_t` because it's not supported by JNA
}

impl From<Vec<CBuiltinEntity>> for CBuiltinEntityArray {
    fn from(input: Vec<CBuiltinEntity>) -> Self {
        Self {
            size: input.len() as i32,
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

unsafe impl Send for DummyWrapper {}
unsafe impl Sync for DummyWrapper {}

pub fn all_builtin_entities() -> CStringArray {
    lazy_static! {
        static ref ALL: DummyWrapper = {
            DummyWrapper(
                BuiltinEntityKind::all()
                    .iter()
                    .map(|l| l.identifier().to_string())
                    .map(|l| CString::new(l).unwrap().into_raw() as *const libc::c_char)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            )
        };
    }

    CStringArray {
        data: ALL.0.as_ptr() as *const *const libc::c_char,
        size: ALL.0.len() as i32,
    }
}

pub fn all_grammar_entities() -> CStringArray {
    lazy_static! {
        static ref ALL: DummyWrapper = {
            DummyWrapper(
                GrammarEntityKind::all()
                    .iter()
                    .map(|l| l.identifier().to_string())
                    .map(|l| CString::new(l).unwrap().into_raw() as *const libc::c_char)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            )
        };
    }

    CStringArray {
        data: ALL.0.as_ptr() as *const *const libc::c_char,
        size: ALL.0.len() as i32,
    }
}

pub fn all_gazetteer_entities() -> CStringArray {
    lazy_static! {
        static ref ALL: DummyWrapper = {
            DummyWrapper(
                BuiltinGazetteerEntityKind::all()
                    .iter()
                    .map(|l| l.identifier().to_string())
                    .map(|l| CString::new(l).unwrap().into_raw() as *const libc::c_char)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            )
        };
    }

    CStringArray {
        data: ALL.0.as_ptr() as *const *const libc::c_char,
        size: ALL.0.len() as i32,
    }
}

pub fn get_builtin_entity_shortname(
    entity_name: *const libc::c_char,
    result: *mut *const libc::c_char,
) -> Result<()> {
    let entity_str = unsafe { CStr::from_ptr(entity_name) }.to_str()?;
    let entity_kind = BuiltinEntityKind::from_identifier(entity_str)?;
    point_to_string(result, entity_kind.to_string())
}
