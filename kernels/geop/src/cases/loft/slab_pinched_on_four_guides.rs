//! A slab lofted through rectangles centred on the z axis, from 24 × 6.4 at z = 0 to 6 × 24.4 at
//! z = 24, its corners running along four guides: the cubic Bézier curve with control points
//! (12, 3.2, 0), (18, −2.8, 8), (15, 0.2, 19), (3, 12.2, 24) and its mirror images in the xz and
//! yz planes and both.
//!
//! Each guide lies in an upright plane (x + y = 15.2 for the first: its control points' x + y is
//! 15.2), so it is drawn there, on a datum plane, as a sketch spline of its four control points.
//! The rectangles are sketched on the xy plane and on a plane 24 up, and lofted guided by the
//! curves. geop's loft takes at most three guides, which fix any affine change of the sections:
//! the guides through three corners of the rectangle set it, and its fourth corner follows.
use crate::shared::paths::{drawn_at, loft, offset_plane, path2d, plane_square_to};
use crate::shared::{Drawing, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut foot = Drawing::new();
    foot.rectangle([-12.0, -3.2], [12.0, 3.2]);
    sketch(b, "foot", xy(), foot)?;
    let top = offset_plane(b, "top", FrameAxis::Z, 24.0)?;
    let mut head = Drawing::new();
    head.rectangle([-3.0, -12.2], [3.0, 12.2]);
    sketch(b, "head", top, head)?;

    let guide = [[12.0, 3.2, 0.0], [18.0, -2.8, 8.0], [15.0, 0.2, 19.0], [3.0, 12.2, 24.0]];
    let mut guides = Vec::new();
    for (id, sx, sy) in [("guide_pp", 1.0, 1.0), ("guide_mp", -1.0, 1.0), ("guide_mm", -1.0, -1.0)] {
        // The upright plane sx·x + sy·y = 15.2 the mirrored guide lies in.
        let (at, normal) = ([sx * 15.2, 0.0, 0.0], [sx, sy, 0.0]);
        let plane = plane_square_to(b, &format!("{id}_plane"), at, normal)?;
        let mut curve = Drawing::new();
        let points = guide.map(|[x, y, z]| curve.point(drawn_at(at, normal, [sx * x, sy * y, z])));
        curve.sketch.add_spline(points.to_vec());
        sketch(b, id, plane, curve)?;
        guides.push(path2d(id));
    }
    loft(b, "slab", &["foot", "head"], guides)
}
