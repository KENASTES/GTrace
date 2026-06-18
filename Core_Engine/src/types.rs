use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum ApertureShape {
    Circle,
    Rectangle,
    Obround,
}

#[derive(Debug, Clone)]
pub struct Aperture {
    pub shape: ApertureShape,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug)]
pub struct SolderPad {
    pub x: f64,
    pub y: f64,
    pub aperture: Aperture,
}

#[derive(Debug)]
pub struct LineSegment {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub thickness: f64,
}

#[derive(Debug)]
pub struct CncState {
    pub current_x: f64,
    pub current_y: f64,
    pub format_decimals: u8,
    pub scale_factor: f64,
    pub unit_scale_in_mm: f64,
    pub apertures: HashMap<i32, Aperture>,
    pub current_aperture: i32,
    pub current_d_code: i32,
    pub traces: Vec<LineSegment>,
    pub pins: Vec<SolderPad>,
}

impl CncState {
    pub fn new() -> Self {
        Self {
            current_x: 0.0,
            current_y: 0.0,
            format_decimals: 6,
            scale_factor: 1_000_000.0,
            unit_scale_in_mm: 1.0,
            apertures: HashMap::new(),
            current_aperture: 0,
            current_d_code: 2,
            traces: Vec::new(),
            pins: Vec::new(),
        }
    }

    pub fn parse_coordinate(&self, raw_val: f64) -> f64 {
        raw_val / self.scale_factor * self.unit_scale_in_mm
    }

    pub fn current_aperture(&self) -> Aperture {
        self.apertures
            .get(&self.current_aperture)
            .cloned()
            .unwrap_or(Aperture {
                shape: ApertureShape::Circle,
                width: 1.5,
                height: 1.5,
            })
    }

    pub fn current_thickness(&self) -> f64 {
        let aperture = self.current_aperture();
        aperture.width.max(aperture.height)
    }
}

impl Default for CncState {
    fn default() -> Self {
        Self::new()
    }
}
