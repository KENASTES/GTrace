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
    mirror_x: i32,
    mirror_y: i32,
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

    apply_preview_mirror(&mut data, mirror_x, mirror_y);

    let json_string = serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string());

    let c_string = CString::new(json_string).unwrap();
    c_string.into_raw()
}

fn apply_preview_mirror(data: &mut PreviewData, mirror_x: i32, mirror_y: i32) {
    if mirror_x != 1 && mirror_y != 1 {
        return;
    }

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for point in data
        .copper_polygons
        .iter()
        .chain(data.toolpaths.iter())
        .flatten()
    {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }

    if !min_x.is_finite() || !max_x.is_finite() || !min_y.is_finite() || !max_y.is_finite() {
        return;
    }

    for point in data
        .copper_polygons
        .iter_mut()
        .chain(data.toolpaths.iter_mut())
        .flatten()
    {
        if mirror_x == 1 {
            point.x = max_x + min_x - point.x;
        }
        if mirror_y == 1 {
            point.y = max_y + min_y - point.y;
        }
    }
}
