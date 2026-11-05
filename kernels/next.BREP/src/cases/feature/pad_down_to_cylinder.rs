//! A cylinder of radius 10 along the x axis from x = −20 to x = 20, and a pad joined to it: the
//! square |x| ≤ 3, |y| ≤ 3 in the plane z = 15 extruded down until it meets the cylinder's side.
//!
//! next.BREP's extrude has no "up to a face" end; the pad is extruded down past the side into the
//! cylinder, to its axis z = 0, and joined: below the side the pad lies inside the cylinder, so the
//! union ends the pad on the side, as the description says.
use crate::solids::boolean::union;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(-20.0, 0.0, 0.0), p(1.0, 0.0, 0.0), 10.0, 40.0)?;
    let square = polygon(&[
        p(-3.0, -3.0, 15.0), p(3.0, -3.0, 15.0), p(3.0, 3.0, 15.0), p(-3.0, 3.0, 15.0),
    ])?;
    let pad = extrude_profile_brep(&square, p(0.0, 0.0, -1.0), 15.0)?;
    Ok(union(&cylinder, &pad)?)
}
