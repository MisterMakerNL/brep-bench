//! The plate from (−20, −20, 0) to (20, 20, 5) with a boss of radius 8 about the z axis standing on
//! it to z = 17, the boss's side drafted 4° for the pull direction +z about the neutral plane
//! z = 5, the plate's top: the kernel's draft makes the side the cone through the boss's circle at
//! z = 5, leaning inward as it rises, and re-trims the boss's top to it.
use crate::solids::boolean::union;
use crate::solids::draft::faces_at;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{draft_faces, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-20.0, -20.0, 0.0), 40.0, 40.0, 5.0)?;
    let boss = make_cylinder_brep(p(0.0, 0.0, 5.0), p(0.0, 0.0, 1.0), 8.0, 12.0)?;
    let solid = union(&plate, &boss)?;
    let side = faces_at(&solid, &[p(8.0, 0.0, 11.0)])?;
    Ok(draft_faces(&solid, &side, p(0.0, 0.0, 1.0), p(0.0, 0.0, 5.0), 4f64.to_radians())?)
}
