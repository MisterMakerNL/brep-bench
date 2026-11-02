//! A C-shaped bracket, the polygon through (x, z) = (0, 0), (40, 0), (40, 6), (6, 6), (6, 24),
//! (40, 24), (40, 30), (0, 30) in the plane y = −10 extruded 20 along +y, with a hole of radius 4
//! on the vertical axis through (28, 0) drilled down through the upper arm and no further.
//!
//! brepkit has no hole feature that drills through to the next face: a hole is a cylinder cut
//! away. So the drill is a cylinder from above the upper arm's top (z = 31) down into the gap
//! between the arms (z = 15), past the arm's underside at z = 24 and short of the lower arm.
use brepkit_operations::extrude::extrude;
use brepkit_topology::builder::make_planar_face;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::cylinder_z;
use crate::solids::{cut, p, v};
use crate::Refusal;

const SECTION: [(f64, f64); 8] = [
    (0.0, 0.0),
    (40.0, 0.0),
    (40.0, 6.0),
    (6.0, 6.0),
    (6.0, 24.0),
    (40.0, 24.0),
    (40.0, 30.0),
    (0.0, 30.0),
];

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let outline: Vec<_> = SECTION.iter().map(|&(x, z)| p(x, -10.0, z)).collect();
    let sketch = make_planar_face(topo, &outline, 1e-7)?;
    let bracket = extrude(topo, sketch, v(0.0, 1.0, 0.0), 20.0)?;
    let drill = cylinder_z(topo, 4.0, (28.0, 0.0), 15.0, 31.0)?;
    Ok(cut(topo, bracket, drill)?)
}
