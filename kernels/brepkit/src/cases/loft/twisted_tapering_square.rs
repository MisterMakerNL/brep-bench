//! A square column tapering through a half turn: brepkit's guided sweep of the square of side 30
//! at the foot up the z axis to z = 40, its turn set by an auxiliary spine and its size by a
//! scale law. The guide turns the section as the corners' spirals do, 4.5° for every unit of
//! height, so a helix about the z axis making half a turn over the height serves: only its
//! direction from the axis at each height steers the section, not its distance (brepkit has a
//! helix but no conical spiral). The scale law takes the side linearly from 30 to 10, about the
//! axis.
use brepkit_operations::helix::make_helix_curve;
use brepkit_operations::sweep::{sweep_with_options, SweepOptions};
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::{polygon, polyline};
use crate::solids::{p, v};
use crate::Refusal;

/// NURBS arcs per turn of the guide: how closely it turns at an even rate.
const SEGMENTS_PER_TURN: usize = 64;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let axis = polyline(&[p(0.0, 0.0, 0.0), p(0.0, 0.0, 40.0)])?;
    let guide = make_helix_curve(
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        15.0,
        80.0,
        0.5,
        SEGMENTS_PER_TURN,
    )?;
    let foot = polygon(
        topo,
        &[
            p(15.0, 15.0, 0.0),
            p(-15.0, 15.0, 0.0),
            p(-15.0, -15.0, 0.0),
            p(15.0, -15.0, 0.0),
        ],
    )?;
    let options = SweepOptions {
        aux_spine: Some(guide),
        // the side 30 − z/2 over the side at the foot, z = 40t up the axis
        scale_law: Some(Box::new(|t: f64| (30.0 - 20.0 * t) / 30.0)),
        ..Default::default()
    };
    Ok(sweep_with_options(topo, foot, &axis, &options)?)
}
