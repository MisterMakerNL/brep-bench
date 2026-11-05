//! The box from the origin to (30, 20, 20), drilled through along x by a hole of radius 4 on the
//! line y = 10, z = 10, then its faces x = 0 and x = 30, where the hole comes out, drafted 8° for
//! the pull direction +z about the neutral plane z = 0: each turns about its bottom edge, leaning
//! inward as it rises, and the kernel's draft re-trims the hole's wall to them in ellipses.
use crate::solids::boolean::cut;
use crate::solids::draft::faces_at;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{draft_faces, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 30.0, 20.0, 20.0)?;
    let drill = make_cylinder_brep(p(-5.0, 10.0, 10.0), p(1.0, 0.0, 0.0), 4.0, 40.0)?;
    let solid = cut(&block, &drill)?;
    // Each face by a point on it clear of the hole.
    let ends = faces_at(&solid, &[p(30.0, 3.0, 3.0), p(0.0, 3.0, 3.0)])?;
    Ok(draft_faces(&solid, &ends, p(0.0, 0.0, 1.0), p(0.0, 0.0, 0.0), 8f64.to_radians())?)
}
