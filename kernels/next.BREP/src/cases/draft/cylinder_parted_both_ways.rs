//! The cylinder of radius 10 about the z axis from z = 0 to z = 20, its side drafted 3° both ways
//! from its parting line, the circle at z = 10: the kernel's parting-line draft splits the side
//! there and drafts the upper half for the pull direction +z and the lower for −z, each a cone
//! through the parting circle leaning inward as it leaves it.
use crate::solids::draft::faces_at;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{draft_faces_parted, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let solid = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 10.0, 20.0)?;
    let side = faces_at(&solid, &[p(10.0, 0.0, 5.0)])?;
    Ok(draft_faces_parted(&solid, &side, p(0.0, 0.0, 1.0), p(0.0, 0.0, 10.0), 3f64.to_radians())?)
}
