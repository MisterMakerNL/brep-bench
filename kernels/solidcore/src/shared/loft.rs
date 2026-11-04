//! What the solidcore implementations of the loft cases share: walls ruled between guides.
use crate::shared::nurbs::{bezier_knots, spline, Face};
use solidcore_record::brep::Surface;

/// The surface ruled between two Bézier curves of one degree, `a` and `b` (their control
/// points), each point of `a` joined straight to the point of `b` at the same parameter: the
/// B-spline of that degree along them and of degree 1 across, its net the two control polygons.
pub fn ruled(a: &[[f64; 3]], b: &[[f64; 3]]) -> Surface {
    let net = a.iter().zip(b).map(|(&p, &q)| vec![p, q]).collect();
    spline(&bezier_knots(a.len() - 1), &bezier_knots(1), net)
}

/// The four guides of a section centred on the z axis with sides parallel to x and y: the
/// Bézier `guide` through its corner with x and y positive, and its mirror images, in order
/// round the axis: (x, y), (−x, y), (−x, −y), (x, −y).
pub fn four_guides(guide: &[[f64; 3]]) -> [Vec<[f64; 3]>; 4] {
    let signs = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)];
    signs.map(|(sx, sy)| guide.iter().map(|p| [sx * p[0], sy * p[1], p[2]]).collect())
}

/// The four walls ruled between neighbouring guides of [`four_guides`].
pub fn walls(guides: &[Vec<[f64; 3]>; 4]) -> Vec<Face> {
    (0..4).map(|i| Face::Surface(ruled(&guides[i], &guides[(i + 1) % 4]))).collect()
}
