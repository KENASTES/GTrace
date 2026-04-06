use std::ffi::CStr;
use std::fs::File;
use std::io::{self, BufRead};
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

    println!("Gtrace Core : Computing File - {}", file_path);
    
    let file = match File::open(file_path){
        Ok(f) => f,
        Err(_e) => {
            println!("Gtrace Core : Failed to open file - {}", file_path);
            return -3;
        } 
    };

    let reader = io::BufReader::new(file);

    println!("Start to read file line by line :");

    for (index, line) in reader.lines().enumerate() {
        let line_content = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        if index < 10 {
            println!("Line {}: {}", index + 1, line_content);
        }

    }

    println!("Gtrace Core: Finished processing file - {}", file_path);

    1
}

#[unsafe(no_mangle)]
pub extern "C" fn add_test(a: i32, b: i32) -> i32 {
    a + b
}