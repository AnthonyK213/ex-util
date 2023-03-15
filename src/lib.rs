use std::ffi::{c_char, CStr, CString};

#[macro_export]
macro_rules! string_parse {
    ($c_buf: expr, $default: expr) => {
        match c_buf_to_string($c_buf) {
            Ok(v) => v,
            Err(_) => return $default,
        }
    };
}

pub fn c_buf_to_string(c_buf: *const c_char) -> Result<String, std::str::Utf8Error> {
    Ok(unsafe { CStr::from_ptr(c_buf) }.to_str()?.to_owned())
}

#[no_mangle]
pub unsafe extern "C" fn string_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    drop(CString::from_raw(s));
}
