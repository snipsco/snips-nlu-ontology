use ffi_utils::CStringArray;
use lazy_static::lazy_static;
use libc;
use snips_nlu_ontology::Language;
use std::ffi::CString;

// We are forced to wrap this Box because lazy_static! require to be Sync but
// ffi's type `*const libc::c_char` isn't
struct DummyWrapper(Box<[*const libc::c_char]>);

unsafe impl Send for DummyWrapper {}
unsafe impl Sync for DummyWrapper {}

pub fn supported_languages() -> CStringArray {
    lazy_static! {
        static ref ALL: DummyWrapper = {
            DummyWrapper(
                Language::all()
                    .iter()
                    .map(|l| l.to_string().to_lowercase())
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
