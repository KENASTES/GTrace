use crate::types::{Aperture, ApertureShape, CncState, LineSegment, SolderPad};
use std::io::BufRead;

fn extract_coordinates(line: &str, prefix: char) -> Option<f64> {
    let start_idx = line.find(prefix)?;
    let rest_of_string = &line[start_idx + 1..];
    let end_idx = rest_of_string
        .find(|c: char| !c.is_ascii_digit() && c != '-' && c != '+')
        .unwrap_or(rest_of_string.len());
    rest_of_string[..end_idx].parse::<f64>().ok()
}

fn parse_aperture_definition(line: &str, state: &mut CncState) {
    if let Some(comma_idx) = line.find(',') {
        let prefix = &line[4..comma_idx];
        let d_code_str: String = prefix.chars().filter(|c| c.is_ascii_digit()).collect();
        let aperture_shape = match prefix.chars().find(|c| c.is_ascii_alphabetic()) {
            Some('C') | Some('c') => ApertureShape::Circle,
            Some('R') | Some('r') => ApertureShape::Rectangle,
            Some('O') | Some('o') => ApertureShape::Obround,
            _ => {
                println!("Unsupported aperture shape in: {}", line);
                return;
            }
        };

        let star_idx = line.find('*').unwrap_or(line.len());
        if star_idx > comma_idx {
            let params = line[comma_idx + 1..star_idx].to_uppercase();
            let mut size_parts = params.split('X').map(|part| part.trim());
            let width_str = size_parts.next().unwrap_or("0");
            let height_str = size_parts.next().unwrap_or(width_str);

            if let (Ok(d_code), Ok(width), Ok(height)) = (
                d_code_str.parse::<i32>(),
                width_str.parse::<f64>(),
                height_str.parse::<f64>(),
            ) {
                let width_mm = width * state.unit_scale_in_mm;
                let height_mm = height * state.unit_scale_in_mm;
                let aperture = Aperture {
                    shape: aperture_shape,
                    width: width_mm,
                    height: height_mm,
                };

                state.apertures.insert(d_code, aperture);
                println!(
                    "Complete aperture scan D{} = {:?} {} x {} mm",
                    d_code, aperture_shape, width_mm, height_mm
                );
            } else {
                println!(
                    "Cant parse the aperture definition D='{}', Width='{}', Height='{}'",
                    d_code_str, width_str, height_str
                );
            }
        }
    }
}

pub fn parse_gerber<R: BufRead>(reader: R, state: &mut CncState) {
    println!("Start to parse the gerber file:");

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(_) => continue,
        };

        let line = line.trim();
        if line.is_empty() || line.starts_with("G04") {
            continue;
        }

        if line.starts_with("%FSLAX") {
            if let Some(decimal_char) = line.chars().nth(7)
                && let Some(decimal_num) = decimal_char.to_digit(10_u32)
            {
                state.format_decimals = decimal_num as u8;
                state.scale_factor = f64::powi(10.0, state.format_decimals as i32);
                println!("Format decimals set to: {}", state.scale_factor);
            }
            continue;
        }

        if line.starts_with("%MOMM") {
            state.unit_scale_in_mm = 1.0;
            continue;
        }

        if line.starts_with("%MOIN") {
            state.unit_scale_in_mm = 25.4;
            continue;
        }

        if line.starts_with("%ADD") {
            println!("Parsing aperture definition: {}", line);
            parse_aperture_definition(line, state);
            continue;
        }

        if line.starts_with('%') {
            continue;
        }

        let old_x = state.current_x;
        let old_y = state.current_y;
        let has_x = extract_coordinates(line, 'X');
        let has_y = extract_coordinates(line, 'Y');
        let d_val = extract_coordinates(line, 'D');

        if let Some(raw_x) = has_x {
            state.current_x = state.parse_coordinate(raw_x);
        }

        if let Some(raw_y) = has_y {
            state.current_y = state.parse_coordinate(raw_y);
        }

        let mut should_execute = false;

        if let Some(dv) = d_val {
            let d_code = dv as i32;

            if d_code >= 10 {
                state.current_aperture = d_code;
                if let Some(size) = state.apertures.get(&d_code) {
                    println!(
                        "Selected aperture D{} {:?} {} x {} mm",
                        d_code, size.shape, size.width, size.height
                    );
                }
            } else {
                state.current_d_code = d_code;
                should_execute = true;
            }
        } else if has_x.is_some() || has_y.is_some() {
            should_execute = true;
        }

        if should_execute {
            if state.current_d_code == 1 {
                state.traces.push(LineSegment {
                    start_x: old_x,
                    start_y: old_y,
                    end_x: state.current_x,
                    end_y: state.current_y,
                    thickness: state.current_thickness(),
                });
            } else if state.current_d_code == 3 {
                let aperture = state.current_aperture();
                state.pins.push(SolderPad {
                    x: state.current_x,
                    y: state.current_y,
                    aperture,
                });
                println!(
                    "Added pin at X({:.3}, Y{:.3}) with diameter {:.4} mm",
                    state.current_x,
                    state.current_y,
                    state.current_thickness()
                );
            }
        }
    }
}
