use std::ffi::CString;
use std::slice;

use libc;

#[repr(C)]
#[derive(Debug)]
pub struct CStringArray {
    pub data: *const *const libc::c_char,
    pub size: libc::c_int, // Note: we can't use `libc::size_t` because it's not supported by JNA
}

impl From<Vec<String>> for CStringArray {
    fn from(input: Vec<String>) -> CStringArray {
        Self {
            size: input.len() as libc::c_int,
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
