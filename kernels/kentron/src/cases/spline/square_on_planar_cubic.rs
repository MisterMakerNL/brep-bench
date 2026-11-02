//! A bar of square section, 20 × 20, bent along the planar cubic Bézier curve with control points
//! (0, 0, 0), (16, 12, 0), (18, 26, 0) and (12, 34, 0): at every point the section is the square
//! of side 20 centred on the curve, square to it, two sides level (z = ±10) and two upright. The
//! ends are flat squares. The curve lies in the xy plane, so kentron's pipe, which turns the
//! section with the curve's tangent and no more, keeps its level sides level all along.
use crate::shared::spline::{bezier, square_to_start};
use kentron::make;
use kentron::math::Vector;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let path = bezier(&[(0.0, 0.0, 0.0), (16.0, 12.0, 0.0), (18.0, 26.0, 0.0), (12.0, 34.0, 0.0)]);
    // the section where the path starts: square to it, two sides level
    let start = square_to_start(&path);
    let (centre, tangent) = (start.origin(), Vector::from(start.z()));
    let up = Vector::new(0.0, 0.0, 10.0);
    let across = up.cross(tangent).normalized() * 10.0;
    let corners: Vec<_> =
        [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)].iter().map(|&(a, b)| centre + across * a + up * b).collect();
    let mut sides = Vec::new();
    for i in 0..4 {
        sides.push(make::edge_between(m, corners[i], corners[(i + 1) % 4])?);
    }
    let rim = make::wire(m, &sides)?;
    let square = make::planar_face(m, rim)?;
    let path = make::edge_from_curve(m, path)?;
    Ok(sweep::pipe(m, square, path)?.shape)
}
