//! A round pipe along three edges of a cube of side 30, mitred at its two corners: a disc of
//! radius 2, square to the first run at the origin, swept along the polyline (0, 0, 0) →
//! (30, 0, 0) → (30, 30, 0) → (30, 30, 30) by brepkit's sweep with mitred corners, which sweeps
//! each run on its own and joins them in the plane bisecting each corner.
use brepkit_operations::sweep::{sweep_with_options, SweepCornerMode, SweepOptions};
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::{disc, polyline};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let path = polyline(&[
        p(0.0, 0.0, 0.0),
        p(30.0, 0.0, 0.0),
        p(30.0, 30.0, 0.0),
        p(30.0, 30.0, 30.0),
    ])?;
    let section = disc(topo, p(0.0, 0.0, 0.0), v(1.0, 0.0, 0.0), 2.0)?;
    let options = SweepOptions {
        corner_mode: SweepCornerMode::Miter,
        ..Default::default()
    };
    Ok(sweep_with_options(topo, section, &path, &options)?)
}
