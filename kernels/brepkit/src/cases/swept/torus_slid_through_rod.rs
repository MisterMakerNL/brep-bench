//! A rod grooved by a wheel slid along it. brepkit sweeps faces, not solids, so the wheel's sweep
//! is built by elementary geometry: a solid slid along a line fills itself at each end and the
//! prism its outline, seen along the slide, sweeps between them. Seen along z, the torus (axis
//! along y, tube centres on a circle of radius 6, tube radius 2) covers the stadium of its tube's
//! two meridian discs in the plane through its centre, at x = 11.5 ± 6, and the band of width 4
//! between them. The wheel at each end, centred at z = −9 and z = 29, lies wholly below and above
//! the rod, so the rod, the cylinder of radius 5 from z = 0 to z = 20, is cut by that prism alone,
//! the stadium at z = −9 extruded 38 along +z.
use brepkit_operations::extrude::extrude;
use brepkit_operations::primitives::make_cylinder;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::stadium;
use crate::solids::{cut, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let rod = make_cylinder(topo, 5.0, 20.0)?;
    let outline = stadium(topo, 11.5, 0.0, -9.0, 6.0, 2.0)?;
    let groove = extrude(topo, outline, v(0.0, 0.0, 1.0), 38.0)?;
    Ok(cut(topo, rod, groove)?)
}
