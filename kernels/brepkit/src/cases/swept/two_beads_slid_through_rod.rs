//! A rod grooved by a wheel of two beads slid along it. brepkit sweeps faces, not solids, so the
//! wheel's sweep is built by elementary geometry: the sweep of a union is the union of the sweeps,
//! and each bead, a torus with its axis along y, slid along z fills itself at each end and the
//! prism of its outline seen along z between them: the stadium of its tube's two meridian discs in
//! the plane through its centre and the band between them. For the first bead (tube centres at
//! radius 6 about (11.5, 0, −9), tube radius 2) that is the stadium about y = 0 of half-width 2;
//! for the second (centre 1.5 further along y, tube radius 1.5) the stadium about y = 1.5 of
//! half-width 1.5. The beads at each end of the slide, 40 along z, lie wholly below and above the
//! rod, so the rod is cut by the two prisms alone.
use brepkit_operations::extrude::extrude;
use brepkit_operations::primitives::make_cylinder;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::stadium;
use crate::solids::{cut, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let rod = make_cylinder(topo, 5.0, 20.0)?;
    let first = stadium(topo, 11.5, 0.0, -9.0, 6.0, 2.0)?;
    let first = extrude(topo, first, v(0.0, 0.0, 1.0), 40.0)?;
    let second = stadium(topo, 11.5, 1.5, -9.0, 6.0, 1.5)?;
    let second = extrude(topo, second, v(0.0, 0.0, 1.0), 40.0)?;
    let rod = cut(topo, rod, first)?;
    Ok(cut(topo, rod, second)?)
}
