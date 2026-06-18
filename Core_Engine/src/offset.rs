use clipper2::{
    EndType, JoinType, Path as ClipperPath, Paths as ClipperPaths, Point as ClipperPoint,
    PointScaler,
};
use geo::{LineString, MultiPolygon, coord};

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
struct Micro;

impl PointScaler for Micro {
    const MULTIPLIER: f64 = 1_000_000.0;
}

fn ring_to_clipper_path(ring: &geo::LineString<f64>) -> ClipperPath<Micro> {
    let coords = ring.coords().collect::<Vec<_>>();
    let mut points = Vec::new();

    for (idx, c) in coords.iter().enumerate() {
        if idx + 1 == coords.len()
            && points.first().is_some_and(|p: &ClipperPoint<Micro>| {
                (p.x() - c.x).abs() < 0.000001 && (p.y() - c.y).abs() < 0.000001
            })
        {
            continue;
        }

        points.push(ClipperPoint::<Micro>::new(c.x, c.y));
    }

    ClipperPath::new(points)
}

fn copper_area_to_clipper_paths(copper_area: &MultiPolygon<f64>) -> ClipperPaths<Micro> {
    let mut paths = Vec::new();

    for poly in copper_area.iter() {
        paths.push(ring_to_clipper_path(poly.exterior()));

        for interior in poly.interiors() {
            paths.push(ring_to_clipper_path(interior));
        }
    }

    ClipperPaths::new(paths)
}

fn clipper_paths_to_lines(paths: &ClipperPaths<Micro>) -> Vec<LineString<f64>> {
    let mut lines = Vec::new();

    for path in paths.iter() {
        let mut coords = path
            .iter()
            .map(|point| coord! { x: point.x(), y: point.y() })
            .collect::<Vec<_>>();

        if coords.len() > 1 && coords.first() != coords.last() {
            coords.push(coords[0]);
        }

        if coords.len() > 1 {
            lines.push(LineString::new(coords));
        }
    }

    lines
}

pub fn generate_isolation_paths(
    copper_area: &MultiPolygon<f64>,
    tool_width_mm: f64,
    clearance_mm: f64,
    isolation_width_mm: f64,
    stepover: f64,
) -> Vec<LineString<f64>> {
    let copper_paths = copper_area_to_clipper_paths(copper_area);
    if copper_paths.is_empty() {
        return Vec::new();
    }

    let first_offset = clearance_mm + tool_width_mm / 2.0;
    let offset_step = (tool_width_mm * stepover).max(tool_width_mm * 0.1);
    let mut offset = first_offset;
    let mut isolation_paths = Vec::new();

    while offset <= isolation_width_mm + 0.000001 {
        let offset_paths = copper_paths
            .inflate(offset, JoinType::Round, EndType::Polygon, 2.0)
            .simplify(0.001, false);
        isolation_paths.extend(clipper_paths_to_lines(&offset_paths));
        offset += offset_step;
    }

    isolation_paths
}
