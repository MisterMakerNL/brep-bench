//! The ball of radius 5 about the origin, its poles on the line along (3, 0, 4), bored along the z
//! axis by a cylinder of radius 3 from z = −6 to z = 6: each rim runs through a pole.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(3.0, 0.0, 4.0))?;
    let drill = make_cylinder_brep(p(0.0, 0.0, -6.0), p(0.0, 0.0, 1.0), 3.0, 12.0)?;
    Ok(cut(&ball, &drill)?)
}
