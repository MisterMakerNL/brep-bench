//! The ball of radius 5 about the origin, its poles on the z axis, less the cap beyond the plane
//! x + z = 5, whose rim runs through the pole (0, 0, 5) across every meridian.
use crate::solids::p;
use crate::solids::singular::keep_toward;
use crate::Refusal;
use brep_kernel::{make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    // keep x + z ≤ 5
    keep_toward(&ball, p(0.0, 0.0, 5.0), p(-1.0, 0.0, -1.0), 20.0)
}
