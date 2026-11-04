//! A closed hollow dome, a plate (the disc of radius 20 about the z axis from z = −2 to z = 0)
//! and on it the hemispherical shell between 18 and 20 from the origin above z = 0, with a post
//! inside: the disc of radius 3 on the cavity's floor extruded up until it meets the inside of
//! the dome, the sphere of radius 18, and joined to the dome.
//!
//! solidcore-ops has no extrusion up to a face, so the post is a cylinder from the floor up past
//! the inner sphere into the shell (to z = 19, short of the outer sphere), united with the dome:
//! the part in the shell is the shell's already.
use crate::bench::{cut, fuse};
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 20.0)?;
    let hollow = cut(ball, make_sphere([0.0, 0.0, 0.0], 18.0)?)?;
    // the shell above z = 0: the hollow ball less everything below
    let shell = cut(hollow, make_box([-25.0, -25.0, -25.0], [50.0, 50.0, 25.0])?)?;
    let plate = make_cylinder([0.0, 0.0, -2.0], 20.0, 2.0)?;
    let dome = fuse(plate, shell)?;
    let post = make_cylinder([0.0, 0.0, 0.0], 3.0, 19.0)?;
    fuse(dome, post)
}
