//! A square bar mitred round a sharp corner: the square 10 × 10, standing in the plane x = 0
//! centred on the origin with two sides horizontal, swept along the polyline (0, 0, 0) →
//! (40, 0, 0) → (20, 20√3, 0) by brepkit's sweep with mitred corners, which sweeps each run on
//! its own and joins them in the plane bisecting the corner.
use brepkit_operations::sweep::{sweep_with_options, SweepCornerMode, SweepOptions};
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::p;
use crate::solids::sweep::{polygon, polyline};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let path = polyline(&[
        p(0.0, 0.0, 0.0),
        p(40.0, 0.0, 0.0),
        p(20.0, 20.0 * 3f64.sqrt(), 0.0),
    ])?;
    let section = polygon(
        topo,
        &[
            p(0.0, -5.0, -5.0),
            p(0.0, 5.0, -5.0),
            p(0.0, 5.0, 5.0),
            p(0.0, -5.0, 5.0),
        ],
    )?;
    let options = SweepOptions {
        corner_mode: SweepCornerMode::Miter,
        ..Default::default()
    };
    Ok(sweep_with_options(topo, section, &path, &options)?)
}
