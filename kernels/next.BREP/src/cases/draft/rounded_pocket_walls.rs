//! The block from the origin to (40, 30, 15) with a pocket cut into its top from (8, 8, 5) to
//! (32, 22, 15), its vertical edges rounded to radius 4, all eight walls drafted 5° for the pull
//! direction +z about the neutral plane z = 5, its floor, leaning away from its middle as they rise:
//! the cutter is the kernel's drafted extrusion of the floor's rounded rectangle, widening 5° as it
//! rises, run on past the block's top.
use crate::solids::boolean::cut;
use crate::solids::draft::rounded_rectangle;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep_draft, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 40.0, 30.0, 15.0)?;
    let floor = rounded_rectangle(8.0, 8.0, 32.0, 22.0, 4.0, 5.0)?;
    let pocket =
        extrude_profile_brep_draft(&floor, p(0.0, 0.0, 1.0), 15.0, -(5f64.to_radians()), None)?;
    Ok(cut(&block, &pocket)?)
}
