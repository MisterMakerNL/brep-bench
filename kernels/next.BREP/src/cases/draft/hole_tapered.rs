//! The block from (−15, −15, 0) to (15, 15, 20), bored through along the z axis by a hole of
//! radius 6 whose wall is drafted 3° for the pull direction +z about the neutral plane z = 0, the
//! floor: the cutter is the kernel's drafted extrusion of the hole's circle at z = 0, widening 3°
//! as it rises, run on past the top.
use crate::solids::boolean::cut;
use crate::solids::draft::circle_halves;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep_draft, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-15.0, -15.0, 0.0), 30.0, 30.0, 20.0)?;
    let circle = circle_halves(p(0.0, 0.0, 0.0), 6.0)?;
    let drill =
        extrude_profile_brep_draft(&circle, p(0.0, 0.0, 1.0), 25.0, -(3f64.to_radians()), None)?;
    Ok(cut(&block, &drill)?)
}
