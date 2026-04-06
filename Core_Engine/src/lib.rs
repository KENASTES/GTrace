use std::ffi::CStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::os::raw::c_char;

#[derive(Debug)]
pub struct CncState {
    pub current_x: f64,
    pub current_y: f64,
    pub is_laser_on: bool,
    pub format_decimals: u8,
    pub scale_factor: f64,
}

impl CncState {
    pub fn new() -> Self {
        CncState {
            current_x: 0.0,
            current_y: 0.0,
            is_laser_on: false,
            format_decimals: 6,
            scale_factor: 1_000_000.0,
        }
    }

    pub fn parse_coordinate(&self, raw_val: f64) -> f64 {
    raw_val / self.scale_factor

    }
}

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

    let mut state = CncState::new();

    println!("Start to prase the garber file :");

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(_) => continue,
        };

        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("G04") {
            continue;
        }

        if line.starts_with("%FSLAX") {
            if let Some(decimal_char) = line.chars().nth(7) {
                if let Some(decimal_num) = decimal_char.to_digit(10_u32) {
                    state.format_decimals = decimal_num as u8;
                    state.scale_factor = f64::powi(10.0, state.format_decimals as i32);

                    println!("Format decimals set to: {}", state.scale_factor);
                }
            }
            continue;
        }

        if !line.starts_with('%') {
            println!("Processing line: {}", line);
        }
    }

    println!("Gtrace Core: Finished processing file - {}", file_path);

    1
}

#[unsafe(no_mangle)]
pub extern "C" fn add_test(a: i32, b: i32) -> i32 {
    a + b
}