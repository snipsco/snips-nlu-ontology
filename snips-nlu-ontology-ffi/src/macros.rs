#[macro_export]
macro_rules! convert_to_c_string_result {
    ($string:expr) => {
        CString::new($string).chain_err(||"Could not convert String to C Repr").map(|s| s.into_raw())
    };
}
