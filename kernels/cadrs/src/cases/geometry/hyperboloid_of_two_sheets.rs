//! The solid inside one sheet of a hyperboloid of two sheets about the z axis, below the plane
//! z = 6: every point with z ≥ 3√(1 + (x² + y²)/16) and z ≤ 6. It is the hyperbola
//! z²/9 − x²/16 = 1 of the xz plane (its branch with z > 0, from its vertex (0, 0, 3) to
//! x = 4√3) turned a whole turn about the z axis, which it meets at its vertex.
//!
//! cadrs's sketch has no hyperbola, so the section is drawn as a wire on the Front (xz) plane:
//! the hyperbola's arc from its vertex (0, 3) to (4√3, 6) as the rational quadratic it is, the
//! top edge back to the axis and the axis down to the vertex. The arc x = 4 sinh t,
//! z = 3 cosh t for 0 ≤ t ≤ T, cosh T = 2, has its end tangents meeting at (4/√3, 3), weighted
//! cosh(T/2) = √(3/2) there. The wire is filled with its plane, and the sheet turned a whole
//! turn about z.
use crate::bench::xz;
use crate::shared::geometry::{turned_about_z, Section};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let rim = 4.0 * 3f64.sqrt();
    let section = Section::on(xz(0.0))
        .conic((0.0, 3.0), (4.0 / 3f64.sqrt(), 3.0), 1.5f64.sqrt(), (rim, 6.0))
        .line((rim, 6.0), (0.0, 6.0))
        .line((0.0, 6.0), (0.0, 3.0));
    turned_about_z(k, &section)
}
