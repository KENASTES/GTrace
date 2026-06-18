use crate::types::{Aperture, ApertureShape, CncState, LineSegment};
use geo::BooleanOps;
use geo::{LineString, MultiPolygon, Polygon, coord};

fn create_circle(x: f64, y: f64, radius: f64) -> Polygon<f64> {
    let mut points = Vec::new();
    let sides = 64;

    for i in 0..sides {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / (sides as f64);
        points.push(coord! {
            x: x + radius * angle.cos(),
            y: y + radius * angle.sin()
        });
    }

    points.push(points[0]);
    Polygon::new(LineString::new(points), vec![])
}

fn create_rectangle(x: f64, y: f64, width: f64, height: f64) -> Polygon<f64> {
    let half_w = width / 2.0;
    let half_h = height / 2.0;

    let p1 = coord! { x: x - half_w, y: y - half_h };
    let p2 = coord! { x: x + half_w, y: y - half_h };
    let p3 = coord! { x: x + half_w, y: y + half_h };
    let p4 = coord! { x: x - half_w, y: y + half_h };

    Polygon::new(LineString::new(vec![p1, p2, p3, p4, p1]), vec![])
}

fn create_obround(x: f64, y: f64, width: f64, height: f64) -> MultiPolygon<f64> {
    if (width - height).abs() < 0.0001 {
        return MultiPolygon::new(vec![create_circle(x, y, width / 2.0)]);
    }

    if width > height {
        let radius = height / 2.0;
        let offset = (width - height) / 2.0;
        let rect = MultiPolygon::new(vec![create_rectangle(x, y, width - height, height)]);
        let left = MultiPolygon::new(vec![create_circle(x - offset, y, radius)]);
        let right = MultiPolygon::new(vec![create_circle(x + offset, y, radius)]);
        return rect.union(&left).union(&right);
    }

    let radius = width / 2.0;
    let offset = (height - width) / 2.0;
    let rect = MultiPolygon::new(vec![create_rectangle(x, y, width, height - width)]);
    let bottom = MultiPolygon::new(vec![create_circle(x, y - offset, radius)]);
    let top = MultiPolygon::new(vec![create_circle(x, y + offset, radius)]);
    rect.union(&bottom).union(&top)
}

fn aperture_to_polygons(x: f64, y: f64, aperture: &Aperture) -> MultiPolygon<f64> {
    match aperture.shape {
        ApertureShape::Circle => MultiPolygon::new(vec![create_circle(x, y, aperture.width / 2.0)]),
        ApertureShape::Rectangle => MultiPolygon::new(vec![create_rectangle(
            x,
            y,
            aperture.width,
            aperture.height,
        )]),
        ApertureShape::Obround => create_obround(x, y, aperture.width, aperture.height),
    }
}

fn line_to_polygon(segment: &LineSegment) -> Polygon<f64> {
    let dx = segment.end_x - segment.start_x;
    let dy = segment.end_y - segment.start_y;
    let length = (dx * dx + dy * dy).sqrt();

    if length < 0.0001 {
        let radius = segment.thickness / 2.0;
        let mut points = Vec::new();
        let sides = 8;

        for i in 0..sides {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (sides as f64);
            points.push(coord! {
                x: segment.start_x + radius * angle.cos(),
                y: segment.start_y + radius * angle.sin()
            });
        }

        points.push(points[0]);
        return Polygon::new(LineString::new(points), vec![]);
    }

    let nx = -dy / length;
    let ny = dx / length;
    let half_t = segment.thickness / 2.0;

    let p1 = coord! {
        x: segment.start_x + nx * half_t,
        y: segment.start_y + ny * half_t
    };

    let p2 = coord! {
        x: segment.start_x - nx * half_t,
        y: segment.start_y - ny * half_t
    };

    let p3 = coord! {
        x: segment.end_x - nx * half_t,
        y: segment.end_y - ny * half_t
    };

    let p4 = coord! {
        x: segment.end_x + nx * half_t,
        y: segment.end_y + ny * half_t
    };

    Polygon::new(LineString::new(vec![p1, p2, p3, p4, p1]), vec![])
}

pub fn build_merged_copper_area(state: &CncState) -> MultiPolygon<f64> {
    let mut polygons: Vec<Polygon<f64>> = Vec::new();

    for trace in &state.traces {
        polygons.push(line_to_polygon(trace));

        let radius = trace.thickness / 2.0;
        polygons.push(create_circle(trace.start_x, trace.start_y, radius));
        polygons.push(create_circle(trace.end_x, trace.end_y, radius));
    }

    for pin in &state.pins {
        polygons.extend(aperture_to_polygons(pin.x, pin.y, &pin.aperture).0);
    }

    let mut merged_area = MultiPolygon::new(Vec::new());
    for poly in polygons {
        merged_area = merged_area.union(&MultiPolygon::new(vec![poly]));
    }

    merged_area
}
