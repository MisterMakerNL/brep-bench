//! An L-shaped shelf bracket: its outline in the plane y = −5 (x and z), lines and one quarter
//! circle of radius 8 about (28, 92) rounding the inside corner, extruded 10 along +y.
use brepkit_operations::extrude::extrude;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::{outline_face, Run};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let at = |x: f64, z: f64| p(x, -5.0, z);
    // The round runs from (20, 92) on the post to (28, 100) on the arm, through the side of its
    // circle toward the corner (20, 100): turning from −x to +z, counter-clockwise about +y.
    let round = Run::Arc {
        centre: at(28.0, 92.0),
        axis: v(0.0, 1.0, 0.0),
    };
    let outline = outline_face(
        topo,
        &[
            (at(0.0, 0.0), Run::Line),
            (at(20.0, 0.0), Run::Line),
            (at(20.0, 92.0), round),
            (at(28.0, 100.0), Run::Line),
            (at(100.0, 100.0), Run::Line),
            (at(100.0, 120.0), Run::Line),
            (at(0.0, 120.0), Run::Line),
        ],
    )?;
    Ok(extrude(topo, outline, v(0.0, 1.0, 0.0), 10.0)?)
}
