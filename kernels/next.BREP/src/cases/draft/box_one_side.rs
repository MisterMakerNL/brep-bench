//! The box from the origin to (20, 12, 10), its side x = 20 drafted 10° for the pull direction +z
//! about the neutral plane z = 0: the kernel's draft turns the side about its bottom edge, leaning
//! inward as it rises, and re-trims the faces beside it.
use crate::solids::draft::faces_at;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{draft_faces, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let solid = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 12.0, 10.0)?;
    let side = faces_at(&solid, &[p(20.0, 6.0, 5.0)])?;
    Ok(draft_faces(&solid, &side, p(0.0, 0.0, 1.0), p(0.0, 0.0, 0.0), 10f64.to_radians())?)
}
