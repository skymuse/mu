use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    pub fn system(process: *const c_char) -> i32;
}

pub fn system_shell(process: &str) {
    if let Ok(process) = CString::new(process) {
        unsafe {
            system(process.as_ptr());
        }
    }
}
