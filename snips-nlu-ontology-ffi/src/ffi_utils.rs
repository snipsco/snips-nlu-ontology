#![allow(non_camel_case_types)]

use std::sync::Mutex;
use std::ffi::CString;
use std::slice;

use libc;

lazy_static! {
    pub static ref LAST_ERROR: Mutex<String> = Mutex::new("".to_string());
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
pub struct CStringArray {
    pub data: *const *const libc::c_char,
    pub size: libc::int32_t, // Note: we can't use `libc::size_t` because it's not supported by JNA
}

impl From<Vec<String>> for CStringArray {
    fn from(input: Vec<String>) -> CStringArray {
        Self {
            size: input.len() as libc::int32_t,
            data: Box::into_raw(input.into_iter()
                .map(|s| CString::new(s).unwrap().into_raw())
                .collect::<Vec<_>>()
                .into_boxed_slice()) as *const *const libc::c_char,
        }
    }
}

impl Drop for CStringArray {
    fn drop(&mut self) {
        let _ = unsafe {
            let y = Box::from_raw(slice::from_raw_parts_mut(self.data as *mut *mut libc::c_char, self.size as usize));
            for p in y.into_iter() {
                CString::from_raw(*p);
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn nlu_ontology_destroy_string_array(ptr: *mut CStringArray) {
    let _ = unsafe { Box::from_raw(ptr) };
}
