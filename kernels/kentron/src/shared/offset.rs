//! What the offset implementations share: a planar face through a polygon, and the L-shaped
//! plate three of them grow or shrink.
use crate::bench::{p, v};
use kentron::math::Point;
use kentron::sweep;
use kentron::topology::{Model, Shape};
use kentron::{make, Result};

/// The planar face bounded by the polygon through `corners`, in order.
pub fn polygon_face(m: &mut Model, corners: &[Point]) -> Result<Shape> {
    let n = corners.len();
    let edges = (0..n).map(|i| make::edge_between(m, corners[i], corners[(i + 1) % n])).collect::<Result<Vec<_>>>()?;
    let wire = make::wire(m, &edges)?;
    make::planar_face(m, wire)
}

/// The L-shaped plate: the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and
/// (0, 15) in the xy plane, extruded 8 along +z.
pub fn l_plate(m: &mut Model) -> Result<Shape> {
    let outline = [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];
    let corners: Vec<Point> = outline.iter().map(|&(x, y)| p(x, y, 0.0)).collect();
    let face = polygon_face(m, &corners)?;
    Ok(sweep::extrude(m, face, v(0.0, 0.0, 8.0))?.shape)
}
