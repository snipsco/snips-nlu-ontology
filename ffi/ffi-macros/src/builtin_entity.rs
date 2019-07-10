#![allow(non_camel_case_types)]

use std::convert::From;
use std::ffi::{CStr, CString};
use std::slice;
use std::str::FromStr;

use ffi_utils::take_back_c_string;
use lazy_static::lazy_static;
use libc;
use serde_json;

use crate::errors::*;
use crate::ontology::*;
use ffi_utils::{point_to_string, CReprOf, CStringArray, RawPointerConverter};
use snips_nlu_ontology::{
    complete_entity_ontology, language_entity_ontology, BuiltinEntity, BuiltinEntityKind,
    BuiltinGazetteerEntityKind, GrammarEntityKind, IntoBuiltinEntityKind, Language,
};

#[repr(C)]
#[derive(Debug)]
pub struct CBuiltinEntity {
    pub entity: CSlotValue,
    pub entity_kind: *const libc::c_char,
    pub value: *const libc::c_char,
    pub range_start: libc::int32_t,
    pub range_end: libc::int32_t,
}

impl From<BuiltinEntity> for CBuiltinEntity {
    fn from(e: BuiltinEntity) -> Self {
        Self {
            entity: CSlotValue::from(e.entity),
            entity_kind: CString::new(e.entity_kind.identifier()).unwrap().into_raw(),
            value: CString::new(e.value).unwrap().into_raw(),
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
        size: ALL.0.len() as libc::int32_t,
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
        size: ALL.0.len() as libc::int32_t,
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
        size: ALL.0.len() as libc::int32_t,
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

pub fn get_supported_grammar_entities(
    language: *const libc::c_char,
    results: *mut *const CStringArray,
) -> Result<()> {
    let language_str = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(&*language_str.to_uppercase())?;
    let entities = GrammarEntityKind::all()
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

pub fn get_supported_builtin_gazetteer_entities(
    language: *const libc::c_char,
    results: *mut *const CStringArray,
) -> Result<()> {
    let language_str = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(&*language_str.to_uppercase())?;
    let entities = BuiltinGazetteerEntityKind::all()
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
    let examples = entity_kind
        .examples(language)
        .into_iter()
        .map(|example| example.to_string())
        .collect::<Vec<_>>();
    let c_examples = CStringArray::c_repr_of(examples)?.into_raw_pointer();
    unsafe {
        *results = c_examples;
    }
    Ok(())
}

pub fn get_complete_entity_ontology_json(ontology_result: *mut *const libc::c_char) -> Result<()> {
    let ontology = serde_json::to_string_pretty(&complete_entity_ontology())?;
    point_to_string(ontology_result, ontology)
}

pub fn get_language_entity_ontology_json(
    language: *const libc::c_char,
    ontology_result: *mut *const libc::c_char,
) -> Result<()> {
    let language_str = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(&*language_str.to_uppercase())?;
    let ontology = serde_json::to_string_pretty(&language_entity_ontology(language))?;
    point_to_string(ontology_result, ontology)
}
