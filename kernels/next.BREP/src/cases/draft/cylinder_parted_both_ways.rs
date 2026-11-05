//! The cylinder of radius 10 about the z axis from z = 0 to z = 20, split at its parting line, the
//! circle at z = 10, each half drafted 3° away from it: the kernel's drafted extrusion of that
//! circle 10 up (pull +z) and 10 down (pull −z), each leaning 3° inward as it leaves the parting
//! line, united along it.
use crate::solids::boolean::union;
use crate::solids::draft::circle_halves;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep_draft, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let draft = 3f64.to_radians();
    let parting = circle_halves(p(0.0, 0.0, 10.0), 10.0)?;
    let upper = extrude_profile_brep_draft(&parting, p(0.0, 0.0, 1.0), 10.0, draft, None)?;
    let lower = extrude_profile_brep_draft(&parting, p(0.0, 0.0, -1.0), 10.0, draft, None)?;
    Ok(union(&upper, &lower)?)
}
