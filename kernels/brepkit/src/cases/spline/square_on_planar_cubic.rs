//! A square bar bent along a cubic Bézier curve: the square of side 20 centred on the curve's start,
//! square to it there, two sides horizontal (z = ±10) and two upright (a flat face on its four
//! corners), swept by brepkit's `sweep` along the cubic Bézier on the case's four control points
//! (brepkit's NURBS). `sweep` carries the section on rotation-minimizing frames, which along a
//! curve in the xy plane keep its horizontal sides horizontal, and closes the ends with the square.
use brepkit_operations::sweep::sweep;
use brepkit_topology::builder::make_planar_face;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{bezier, TOL};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let path = bezier(&[
        p(0.0, 0.0, 0.0),
        p(16.0, 12.0, 0.0),
        p(18.0, 26.0, 0.0),
        p(12.0, 34.0, 0.0),
    ])?;
    // the curve leaves the origin along (4, 3, 0); across it, in the xy plane, is (−3, 4, 0)/5
    let across = v(-0.6, 0.8, 0.0);
    let (up, start) = (v(0.0, 0.0, 10.0), path.evaluate(0.0));
    let corners = [
        start - across * 10.0 - up,
        start + across * 10.0 - up,
        start + across * 10.0 + up,
        start - across * 10.0 + up,
    ];
    let section = make_planar_face(topo, &corners, TOL)?;
    Ok(sweep(topo, section, &path)?)
}
