//! A sheet, the parabolic cylinder z = x²/8 for −4 ≤ x ≤ 4 and 0 ≤ y ≤ 10, thickened by 1 to its
//! concave side, up toward the focus. The sheet is the quadratic B-spline of the xz plane through
//! the control points (−4, 2), (0, −2) and (4, 2) — the parabola, exactly — extruded 10 along +y
//! as a face: the B-spline surface of degree 2 by 1 the case gives, its normals up. geop offsets
//! a surface that is no plane or revolution by fitting it, within a proven bound.
use crate::shared::surfaces::{Side, sheet_extrude, thicken};
use crate::shared::{Drawing, sketch, zx, zx_at};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut parabola = Drawing::new();
    let net = [(-4.0, 2.0), (0.0, -2.0), (4.0, 2.0)].map(|(x, z)| parabola.point(zx_at(x, z)));
    parabola.sketch.add_spline(net.to_vec());
    sketch(b, "parabola", zx(), parabola)?;
    sheet_extrude(b, "sheet", "parabola", 10.0)?;
    let trough = b.face([0.0, 5.0, 0.0])?;
    thicken(b, "trough", &trough, 1.0, Side::Along)
}
