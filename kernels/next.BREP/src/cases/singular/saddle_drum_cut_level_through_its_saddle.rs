//! The drum of `geometry/saddle-drum` (the block under the bilinear saddle z = 10 + xy/8 in common
//! with the cylinder of radius 8 about the z axis) less a block above the plane z = 10, tangent to
//! the saddle at its saddle point, reaching past the drum on every side but its floor.
use crate::solids::boolean::{common, cut};
use crate::solids::geometry::under_saddle;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = under_saddle()?;
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 8.0, 30.0)?;
    let drum = common(&block, &cylinder)?;
    let above = make_box_brep(p(-20.0, -20.0, 10.0), 40.0, 40.0, 30.0)?;
    Ok(cut(&drum, &above)?)
}
