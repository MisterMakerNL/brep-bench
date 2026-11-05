//! The box from the origin to (30, 20, 20), drilled through along x by a hole of radius 4 on the
//! line y = 10, z = 10, then its faces x = 0 and x = 30, where the hole comes out, drafted 8° for
//! the pull direction +z about the neutral plane z = 0: each turns about its bottom edge, leaning
//! inward as it rises, and the hole's wall is trimmed by them in ellipses.
use crate::solids::boolean::cut;
use crate::solids::draft::draft_side;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 30.0, 20.0, 20.0)?;
    let drill = make_cylinder_brep(p(-5.0, 10.0, 10.0), p(1.0, 0.0, 0.0), 4.0, 40.0)?;
    let mut solid = cut(&block, &drill)?;
    // Each face by a point on it clear of the hole, and its outward normal.
    for (on_face, normal) in [
        (p(30.0, 3.0, 3.0), p(1.0, 0.0, 0.0)),
        (p(0.0, 3.0, 3.0), p(-1.0, 0.0, 0.0)),
    ] {
        solid = draft_side(&solid, on_face, normal, 0.0, 8f64.to_radians())?;
    }
    Ok(solid)
}
