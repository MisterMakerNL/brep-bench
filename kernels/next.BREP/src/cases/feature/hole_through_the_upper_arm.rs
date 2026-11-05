//! A C-shaped bracket, the polygon (x, z) = (0, 0), (40, 0), (40, 6), (6, 6), (6, 24), (40, 24),
//! (40, 30), (0, 30) in the plane y = −10 extruded along +y to y = 10, with a hole of radius 4 on
//! the vertical axis through (28, 0) drilled down through the upper arm only, from its top z = 30
//! into the gap below it. The drill runs from above the arm down to z = 15, which is in the gap, so
//! it comes out into the gap and never reaches the lower arm.
use crate::solids::boolean::cut;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let at = |x: f64, z: f64| p(x, -10.0, z);
    let section = polygon(&[
        at(0.0, 0.0), at(40.0, 0.0), at(40.0, 6.0), at(6.0, 6.0),
        at(6.0, 24.0), at(40.0, 24.0), at(40.0, 30.0), at(0.0, 30.0),
    ])?;
    let bracket = extrude_profile_brep(&section, p(0.0, 1.0, 0.0), 20.0)?;
    let drill = make_cylinder_brep(p(28.0, 0.0, 15.0), p(0.0, 0.0, 1.0), 4.0, 20.0)?;
    Ok(cut(&bracket, &drill)?)
}
