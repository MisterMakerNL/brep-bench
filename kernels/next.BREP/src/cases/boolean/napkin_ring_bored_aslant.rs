//! A ball of radius 6.528 about (18.676, −15.427, 14.527), bored right through along the line
//! through its centre in the direction (0.448, −0.297, −0.844) by a hole of radius 3.408: the drill
//! a cylinder from 2R before the centre to 2R past it.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, make_sphere_brep, BrepSolid};

const RADIUS: f64 = 6.528;

pub fn build() -> Result<BrepSolid, Refusal> {
    let centre = p(18.676, -15.427, 14.527);
    let along = p(0.448, -0.297, -0.844).normalized()?;
    let ball = make_sphere_brep(centre, RADIUS, p(0.0, 0.0, 1.0))?;
    let drill = make_cylinder_brep(centre.sub(along.scale(2.0 * RADIUS)), along, 3.408, 4.0 * RADIUS)?;
    Ok(cut(&ball, &drill)?)
}
