//! A disc of radius 2 swept a quarter turn along an arc of radius 10 about the z axis: the disc
//! stands square to the arc at (10, 0, 0), and brepkit's sweep carries it along the arc's exact
//! rational NURBS on rotation-minimizing frames, which on a plane arc keep it square to the path.
use std::f64::consts::FRAC_PI_2;

use brepkit_operations::sweep::sweep;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::{arc, disc};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    // the arc from (10, 0, 0) to (0, 10, 0), right-handed about z
    let path = arc(
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        p(10.0, 0.0, 0.0),
        FRAC_PI_2,
    )?;
    // the section where the arc starts, square to its tangent there, +y
    let section = disc(topo, p(10.0, 0.0, 0.0), v(0.0, 1.0, 0.0), 2.0)?;
    Ok(sweep(topo, section, &path)?)
}
