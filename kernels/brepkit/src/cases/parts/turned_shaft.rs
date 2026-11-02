//! A stepped shaft turned about the z axis: its half section in the xz plane, lines and one
//! quarter circle of radius 4 about (16, 64) rounding the shoulder's inside corner, closed down
//! the axis and revolved a full turn about z.
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;
use std::f64::consts::TAU;

use crate::solids::contact::{outline_face, Run};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let at = |x: f64, z: f64| p(x, 0.0, z);
    // The round runs from (16, 60) on the shoulder to (12, 64) on the thin part, through the side
    // of its circle toward the corner (12, 60): turning from −z to −x, counter-clockwise about +y.
    let round = Run::Arc {
        centre: at(16.0, 64.0),
        axis: v(0.0, 1.0, 0.0),
    };
    let section = outline_face(
        topo,
        &[
            (at(0.0, 0.0), Run::Line),
            (at(20.0, 0.0), Run::Line),
            (at(20.0, 60.0), Run::Line),
            (at(16.0, 60.0), round),
            (at(12.0, 64.0), Run::Line),
            (at(12.0, 160.0), Run::Line),
            (at(0.0, 160.0), Run::Line),
        ],
    )?;
    Ok(revolve(
        topo,
        section,
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        TAU,
    )?)
}
