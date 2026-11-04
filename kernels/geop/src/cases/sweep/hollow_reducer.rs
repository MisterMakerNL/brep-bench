//! A hollow square tube tapering along the y axis from y = 0 to y = 40: outside from 24 × 24 to
//! 12 × 12, its hole from 20 × 20 to 8 × 8, each corner running straight from end to end.
//!
//! geop lofts through single loops, not through a section with a hole, so the tube is the solid
//! less its cavity: the outside lofted from its square on the xz plane to its square on a plane
//! 40 along y — two sections, joined by flat ruled walls — and the hole lofted the same way and
//! cut from it.
use crate::shared::paths::{loft, offset_plane};
use crate::shared::{Drawing, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_boolean::BooleanArgs;
use geop_ops_model::{BooleanOp, solid};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let far = offset_plane(b, "far", FrameAxis::Y, 40.0)?;
    for (id, plane, half) in [
        ("outside_near", zx(), 12.0),
        ("outside_far", far.clone(), 6.0),
        ("hole_near", zx(), 10.0),
        ("hole_far", far, 4.0),
    ] {
        let mut square = Drawing::new();
        square.rectangle(zx_at(-half, -half), zx_at(half, half));
        sketch(b, id, plane, square)?;
    }
    loft(b, "outside", &["outside_near", "outside_far"], Vec::new())?;
    loft(b, "hole", &["hole_near", "hole_far"], Vec::new())?;
    b.step(
        "tube",
        BooleanArgs { a: solid("loft(outside)"), b: solid("loft(hole)"), op: BooleanOp::Difference },
    )
}
