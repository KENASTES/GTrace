mod exporter;
mod geometry;
mod offset;
mod parser;
mod types;

use crate::exporter::write_gcode;
use crate::geometry::build_merged_copper_area;
use crate::offset::generate_isolation_paths;
use crate::parser::parse_gerber;
use crate::types::CncState;
use std::ffi::CStr;
use std::fs::File;
use std::io::BufReader;
use std::os::raw::c_char;

const DEFAULT_TOOL_WIDTH_MM: f64 = 0.20;
const DEFAULT_CLEARANCE_MM: f64 = 0.05;
const DEFAULT_STEPOVER: f64 = 0.80;
const FALLBACK_ISOLATION_WIDTH_MM: f64 = 0.60;

#[unsafe(no_mangle)]
/// # Safety
///
/// `input_path_ptr` and `out_path_ptr` must be valid, non-null, null-terminated C strings
/// for the duration of this call.
pub unsafe extern "C" fn process_gerber_to_gcode(
    input_path_ptr: *const c_char,
    out_path_ptr: *const c_char,
    feed_rate: i32,
    laser_power: i32,
    mirror_x: i32,
    isolation_width_mm: f64,
) -> i32 {
    if input_path_ptr.is_null() || out_path_ptr.is_null() {
        return -1;
    }

    let c_input = unsafe { CStr::from_ptr(input_path_ptr) };
    let input_path = match c_input.to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };

    let c_out = unsafe { CStr::from_ptr(out_path_ptr) };
    let out_path = match c_out.to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };

    println!("Gtrace Core : Computing File - {}", input_path);

    let file = match File::open(input_path) {
        Ok(f) => f,
        Err(_) => {
            println!("Gtrace Core : Failed to open file - {}", input_path);
            return -3;
        }
    };

    let mut out_file = match File::create(out_path) {
        Ok(f) => f,
        Err(_) => return -4,
    };

    let reader = BufReader::new(file);
    let mut state = CncState::new();
    parse_gerber(reader, &mut state);

    println!("Generating Polygon data from line segments...");
    let merged_area = build_merged_copper_area(&state);
    println!(
        "Polygon merged complete. Total merged polygons: {}",
        merged_area.0.len()
    );

    println!("Generating isolation offset toolpaths...");
    let first_offset = DEFAULT_CLEARANCE_MM + DEFAULT_TOOL_WIDTH_MM / 2.0;
    let requested_isolation_width = if isolation_width_mm.is_finite() && isolation_width_mm > 0.0 {
        isolation_width_mm.max(first_offset)
    } else {
        FALLBACK_ISOLATION_WIDTH_MM
    };

    let isolation_paths = generate_isolation_paths(
        &merged_area,
        DEFAULT_TOOL_WIDTH_MM,
        DEFAULT_CLEARANCE_MM,
        requested_isolation_width,
        DEFAULT_STEPOVER,
    );

    println!(
        "Start to generate Gcode from {} isolation paths using {:.4} mm isolation width...",
        isolation_paths.len(),
        requested_isolation_width
    );

    if write_gcode(
        &mut out_file,
        &isolation_paths,
        feed_rate,
        laser_power,
        mirror_x,
    )
    .is_err()
    {
        return -5;
    }

    println!("Gtrace Core: Finished processing file - {}", out_path);
    println!("Finished Store the trace data {} line", state.traces.len());
    println!("Finished Store the pin data {} line", state.pins.len());

    1
}
