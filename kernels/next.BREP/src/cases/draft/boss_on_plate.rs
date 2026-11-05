//! The plate from (−20, −20, 0) to (20, 20, 5) with a boss of radius 8 about the z axis standing on
//! it to z = 17, the boss's side drafted 4° for the pull direction +z about the neutral plane
//! z = 5: the kernel's drafted extrusion of the boss's circle, 12 up from the plate's top, leaning
//! 4° inward as it rises, united with the plate.
use crate::solids::boolean::union;
use crate::solids::draft::circle_halves;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep_draft, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-20.0, -20.0, 0.0), 40.0, 40.0, 5.0)?;
    let circle = circle_halves(p(0.0, 0.0, 5.0), 8.0)?;
    let boss = extrude_profile_brep_draft(&circle, p(0.0, 0.0, 1.0), 12.0, 4f64.to_radians(), None)?;
    Ok(union(&plate, &boss)?)
}
