use std::ffi::CStr;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn process_gerber_to_gcode(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1; 
    }

    let c_str = unsafe { CStr::from_ptr(path_ptr) };
    
    let file_path = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -2, 
    };

    println!("Gtrace Core: Computing File - {}", file_path);
    
    let operate_success = true; 

    if operate_success {
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn add_test(a: i32, b: i32) -> i32 {
    a + b
}