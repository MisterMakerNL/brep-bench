//! A closed hollow dome, the disk of radius 20 from z = −2 to z = 0 and on it the hemispherical
//! shell between 18 and 20 from the origin, its cavity the half ball of radius 18 above the plate;
//! and a post, the disk of radius 3 on the cavity's floor extruded up until it meets the inside of
//! the dome, joined to it.
//!
//! The half balls are the kernel's balls (poles on the z axis) less the block under z = 0. The
//! dome is the plate joined to the half ball of radius 20, less the half ball of radius 18.
//! next.BREP's extrude has no "up to a face" end; the post is extruded from the floor up to
//! z = 19, past the cavity's sphere into the dome's wall (within 3 of the axis the wall lies
//! between z = √315 and above z = 19.77), and joined, so it ends on the inside of the dome.
use crate::solids::boolean::{cut, union};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, make_sphere_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, z) = (p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let below = make_box_brep(p(-30.0, -30.0, -30.0), 60.0, 60.0, 30.0)?;
    let half_ball = |r: f64| -> Result<BrepSolid, Refusal> {
        Ok(cut(&make_sphere_brep(o, r, z)?, &below)?)
    };
    let plate = make_cylinder_brep(p(0.0, 0.0, -2.0), z, 20.0, 2.0)?;
    let dome = cut(&union(&plate, &half_ball(20.0)?)?, &half_ball(18.0)?)?;
    let post = make_cylinder_brep(o, z, 3.0, 19.0)?;
    Ok(union(&dome, &post)?)
}
