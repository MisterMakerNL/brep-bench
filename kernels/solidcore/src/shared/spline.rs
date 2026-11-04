//! What the solidcore implementations of the spline cases share.
use crate::shared::nurbs::bezier_knots;
use crate::Refusal;
use solidcore_ops::sweep::{sweep_placed, Curve, SweepFrame};
use solidcore_ops::Body;

/// The cubic Bézier of `points`, as a path.
pub fn cubic(points: [[f64; 3]; 4]) -> Curve {
    Curve::Nurbs { degree: 3, points: points.iter().map(|p| [p[0], p[1], p[2], 1.0]).collect(), knots: bezier_knots(3) }
}

/// `section`, placed square to `path` at its start, swept along it turning with it, the ends
/// capped: the kernel's sweep along a curve.
pub fn swept(section: &[Curve], path: Curve) -> Result<Body, Refusal> {
    let (body, _, _) = sweep_placed(section, &[path], SweepFrame::Follow, true)?;
    Ok(body)
}
