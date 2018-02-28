#![allow(non_camel_case_types)]

use std::sync::Mutex;
use std::ffi::CString;
use std::slice;

use libc;

use errors::*;

lazy_static! {
    pub(crate) static ref LAST_ERROR: Mutex<String> = Mutex::new("".to_string());
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
            match ::ffi_utils::LAST_ERROR.lock() {
                Ok(mut guard) => *guard = msg,
                Err(_) => () /* curl up and cry */
            }
            CResult::RESULT_KO
        }
    }}
}

#[repr(C)]
#[derive(Debug)]
pub struct CStringArray {
    pub data: *const *const libc::c_char,
    pub size: libc::int32_t, // Note: we can't use `libc::size_t` because it's not supported by JNA
}

impl From<Vec<String>> for CStringArray {
    fn from(input: Vec<String>) -> CStringArray {
        Self {
            size: input.len() as libc::int32_t,
            data: Box::into_raw(
                input
                    .into_iter()
                    .map(|s| CString::new(s).unwrap().into_raw())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ) as *const *const libc::c_char,
        }
    }
}

impl Drop for CStringArray {
    fn drop(&mut self) {
        let _ = unsafe {
            let y = Box::from_raw(slice::from_raw_parts_mut(
                self.data as *mut *mut libc::c_char,
                self.size as usize,
            ));
            for p in y.into_iter() {
                CString::from_raw(*p);
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn nlu_ontology_get_last_error(error: *mut *const libc::c_char) -> CResult {
    wrap!(get_last_error(error))
}

#[no_mangle]
pub extern "C" fn nlu_ontology_destroy_string_array(ptr: *mut CStringArray) -> CResult {
   wrap!(destroy(ptr))
}

#[no_mangle]
pub extern "C" fn nlu_ontology_destroy_string(ptr: *mut libc::c_char) -> CResult {
    wrap!(destroy_string(ptr))
}

fn get_last_error(error: *mut *const libc::c_char) -> OntologyResult<()> {
    let last_error = LAST_ERROR.lock().map_err(|e| format!("Can't retrieve last error: {}", e))?.clone();
    let c_last_error = CString::new(last_error).unwrap().into_raw(); // String cannot contain 0
    unsafe {
        *error = c_last_error;
    }
    Ok(())
}

pub(crate) fn destroy_string(string: *mut libc::c_char) -> OntologyResult<()> {
    unsafe {
        let _ = ::std::ffi::CString::from_raw(string);
    }
    Ok(())
}

pub(crate) fn destroy<T>(ptr: *mut T) -> OntologyResult<()> {
    unsafe {
        let _ = Box::from_raw(ptr);
    }
    Ok(())
}
