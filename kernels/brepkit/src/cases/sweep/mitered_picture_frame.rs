//! A picture frame: the rebated moulding swept round the closed rectangle with corners
//! (±30, ±20, 0), mitred at its corners, by brepkit's sweep with mitred corners. The path starts
//! at the middle of the side y = −20 and runs anticlockwise about +z, so the section, standing in
//! the plane x = 0 there, lies on its outer side, toward −y: the point (s, z) of the section is
//! (0, −20 − s, z).
use brepkit_operations::sweep::{sweep_with_options, SweepCornerMode, SweepOptions};
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::p;
use crate::solids::sweep::{polygon, polyline};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let path = polyline(&[
        p(0.0, -20.0, 0.0),
        p(30.0, -20.0, 0.0),
        p(30.0, 20.0, 0.0),
        p(-30.0, 20.0, 0.0),
        p(-30.0, -20.0, 0.0),
        p(0.0, -20.0, 0.0),
    ])?;
    // the moulding's section (s, z): a bar 8 × 6 outside the path, a rebate 3 × 2 out of its
    // inner bottom edge
    let section: Vec<_> = [
        (0.0, 2.0),
        (3.0, 2.0),
        (3.0, 0.0),
        (8.0, 0.0),
        (8.0, 6.0),
        (0.0, 6.0),
    ]
    .iter()
    .map(|&(s, z)| p(0.0, -20.0 - s, z))
    .collect();
    let section = polygon(topo, &section)?;
    let options = SweepOptions {
        corner_mode: SweepCornerMode::Miter,
        ..Default::default()
    };
    Ok(sweep_with_options(topo, section, &path, &options)?)
}
