use geo::{LineString, MultiPolygon};
use serde::Serialize;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Serialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize)]
pub struct PreviewData {
    pub copper_polygons: Vec<Vec<Point2D>>,
    pub toolpaths: Vec<Vec<Point2D>>,
}

pub fn generate_json_preview(
    copper_area: &MultiPolygon<f64>,
    isolation_paths: &[LineString<f64>],
) -> *mut c_char {
    let mut data = PreviewData {
        copper_polygons: Vec::new(),
        toolpaths: Vec::new(),
    };

    for poly in copper_area.iter() {
        let mut ring = Vec::new();
        for c in poly.exterior().coords() {
            ring.push(Point2D { x: c.x, y: c.y });
        }
        data.copper_polygons.push(ring);

        for interior in poly.interiors() {
            let mut hole = Vec::new();
            for c in interior.coords() {
                hole.push(Point2D { x: c.x, y: c.y });
            }
            data.copper_polygons.push(hole);
        }
    }

    for path in isolation_paths {
        let mut line = Vec::new();
        for c in path.coords() {
            line.push(Point2D { x: c.x, y: c.y });
        }
        data.toolpaths.push(line);
    }

    let json_string = serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string());

    let c_string = CString::new(json_string).unwrap();
    c_string.into_raw()
}
