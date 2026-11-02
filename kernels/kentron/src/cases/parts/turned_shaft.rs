//! A stepped shaft on the z axis, the section in the xz plane turned a full turn about the z axis.
//! The section runs (x, z) from (0, 0) out to (20, 0), up to (20, 60), in to (16, 60), round a
//! quarter circle of radius 4 centred at (16, 64) to (12, 64), up to (12, 160), in to (0, 160) and
//! down the axis back to (0, 0): a cylinder of radius 20 from z = 0 to 60 and one of radius 12 on
//! to z = 160, the inside corner at the shoulder rounded to radius 4. The section is drawn with
//! kentron's profile pen, the round an arc tangent to the shoulder turning right through 90°.
use crate::shared::parts::xz_plane;
use kentron::math::Axis;
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Arc, Start, To};
    let outline = [
        Start(0.0, 0.0),
        To(20.0, 0.0),
        To(20.0, 60.0),
        To(16.0, 60.0),
        // centre on the right of the pen, heading in along −x: (16, 64)
        Arc { radius: -4.0, angle: 90.0 },
        To(12.0, 160.0),
        To(0.0, 160.0),
    ];
    let section = sketch::profile(m, xz_plane(0.0), &outline, Outcome::Face)?;
    Ok(sweep::revolve(m, section, Axis::Z, TAU)?.shape)
}
