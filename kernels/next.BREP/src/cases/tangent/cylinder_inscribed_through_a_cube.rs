//! A cube of side 10 about the origin, united with the cylinder of radius 5 on the x axis from
//! x = −10 to x = 10, inscribed in it: tangent to its faces y = ±5 and z = ±5 along their middle
//! lines. The cylinder is turned about its own axis so that its seam lies on the line y = z of its
//! wall, 45° from the lines of contact.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::solids::singular::turned;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(-5.0, -5.0, -5.0), 10.0, 10.0, 10.0)?;
    let cylinder = make_cylinder_brep(p(-10.0, 0.0, 0.0), p(1.0, 0.0, 0.0), 5.0, 20.0)?;
    // the kernel's seam on an x-axis cylinder runs along +z; turn it to (0, 1, 1)/√2
    let cylinder = turned(&cylinder, p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0),
                          (-45.0f64).to_radians(), p(0.0, 0.0, 0.0))?;
    Ok(union(&cube, &cylinder)?)
}
