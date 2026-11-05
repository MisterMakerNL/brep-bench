//! The cube from (−5, −5, −10) to (5, 5, 0) in common with the half-space above the plane
//! z = 10⁻⁶·y, which crosses its top face along the x axis: a wedge under the half y < 0.
use crate::solids::p;
use crate::solids::singular::keep_toward;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(-5.0, -5.0, -10.0), 10.0, 10.0, 10.0)?;
    // z − 10⁻⁶·y ≥ 0
    keep_toward(&cube, p(0.0, 0.0, 0.0), p(0.0, -1e-6, 1.0), 20.0)
}
