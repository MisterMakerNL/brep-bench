//! A capsule's end, the cylinder of radius 5 on the z axis from z = 0 to z = 10 capped by the half
//! ball of radius 5 about (0, 0, 10), shelled inward to a wall 1 thick, open at its hemisphere:
//! the side moved in, the cylinder of radius 4, runs on into the dome until it meets the sphere at
//! z = 13.
use crate::bench::{face_through, fuse, order};
use crate::Refusal;
use solidcore_ops::{make_cylinder, make_sphere, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let side = make_cylinder([0.0, 0.0, 0.0], 5.0, 10.0)?;
    let dome = make_sphere([0.0, 0.0, 10.0], 5.0)?;
    let capsule = fuse(side, dome)?;
    let hemisphere = face_through(&capsule, [0.0, 0.0, 15.0])?;
    shell(&capsule, &order(&capsule)?, &[hemisphere], -1.0)?;
    Ok(capsule)
}
