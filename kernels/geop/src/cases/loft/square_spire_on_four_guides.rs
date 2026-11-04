//! A square spire lofted from the square 18 × 18 on the xy plane, centred on the z axis, to the
//! point (0, 0, 24), its corners running along four ridges: the cubic Bézier curve with control
//! points (9, 9, 0), (15, 15, 8), (12, 12, 19), (0, 0, 24), in the upright plane x = y, and its
//! mirror images in the xz and yz planes and both.
//!
//! Each ridge is drawn in its upright plane through the z axis, on a datum plane, as a sketch
//! spline of its four control points. The square is sketched on the xy plane, the apex as a
//! sketch of one point on a plane 24 up, and the square lofted to it guided by the ridges. geop's
//! loft takes at most three guides, which fix any affine change of the sections: the ridges
//! through three corners of the square set it, and its fourth corner follows. (geop runs guides
//! only between profiles, so it refuses a loft guided into a point; its sweep guided by a rail,
//! the square swept up the z axis scaled by its corner's ridge, refuses too, the rail squeezing
//! the profile to nothing at the apex.)
use crate::shared::paths::{drawn_at, loft, offset_plane, path2d, plane_square_to};
use crate::shared::{Drawing, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut foot = Drawing::new();
    foot.rectangle([-9.0, -9.0], [9.0, 9.0]);
    sketch(b, "foot", xy(), foot)?;
    let top = offset_plane(b, "top", FrameAxis::Z, 24.0)?;
    let mut apex = Drawing::new();
    apex.point([0.0, 0.0]);
    sketch(b, "apex", top, apex)?;

    let ridge = [[9.0, 9.0, 0.0], [15.0, 15.0, 8.0], [12.0, 12.0, 19.0], [0.0, 0.0, 24.0]];
    let mut guides = Vec::new();
    for (id, sx, sy) in [("ridge_pp", 1.0, 1.0), ("ridge_mp", -1.0, 1.0), ("ridge_mm", -1.0, -1.0)] {
        // The upright plane through the z axis and the corner (sx·9, sy·9, 0).
        let (at, normal) = ([0.0, 0.0, 0.0], [sy, -sx, 0.0]);
        let plane = plane_square_to(b, &format!("{id}_plane"), at, normal)?;
        let mut curve = Drawing::new();
        let points = ridge.map(|[x, y, z]| curve.point(drawn_at(at, normal, [sx * x, sy * y, z])));
        curve.sketch.add_spline(points.to_vec());
        sketch(b, id, plane, curve)?;
        guides.push(path2d(id));
    }
    loft(b, "spire", &["foot", "apex"], guides)
}
