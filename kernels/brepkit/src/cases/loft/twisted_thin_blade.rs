//! A thin blade twisted through a whole turn: brepkit's guided sweep of the rectangle 40 × 0.4 up
//! the z axis to z = 60, its turn set by an auxiliary spine, the helix about the z axis rising 60
//! in a turn, as its corners' helices do: the guide's direction from the axis at each height
//! steers the section, 6° for every unit of height.
use brepkit_operations::helix::make_helix_curve;
use brepkit_operations::sweep::sweep_guided;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::{polygon, polyline};
use crate::solids::{p, v};
use crate::Refusal;

/// NURBS arcs per turn of the guide: how closely it turns at an even rate.
const SEGMENTS_PER_TURN: usize = 64;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let axis = polyline(&[p(0.0, 0.0, 0.0), p(0.0, 0.0, 60.0)])?;
    let guide = make_helix_curve(
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        20.0,
        60.0,
        1.0,
        SEGMENTS_PER_TURN,
    )?;
    let foot = polygon(
        topo,
        &[
            p(20.0, 0.2, 0.0),
            p(-20.0, 0.2, 0.0),
            p(-20.0, -0.2, 0.0),
            p(20.0, -0.2, 0.0),
        ],
    )?;
    Ok(sweep_guided(topo, foot, &axis, guide)?)
}
