//! A cylinder of radius 10 along the x axis, from x = −20 to 20, with a square pad on top: the
//! square |x| ≤ 3, |y| ≤ 3 in the plane z = 15, extruded down until it meets the cylinder's side,
//! and joined to it.
//!
//! brepkit's extrude runs a given distance and has no extrusion up to a face. So the square is
//! extruded down 15, to the cylinder's axis: everything of it below the cylinder's side lies
//! inside the cylinder, and the union leaves the pad ending on the side.
use brepkit_operations::extrude::extrude;
use brepkit_topology::builder::make_planar_face;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::cylinder_x;
use crate::solids::{fuse, p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cylinder = cylinder_x(topo, 10.0, (0.0, 0.0), -20.0, 20.0)?;
    let square = [
        p(-3.0, -3.0, 15.0),
        p(3.0, -3.0, 15.0),
        p(3.0, 3.0, 15.0),
        p(-3.0, 3.0, 15.0),
    ];
    let sketch = make_planar_face(topo, &square, 1e-7)?;
    let pad = extrude(topo, sketch, v(0.0, 0.0, -1.0), 15.0)?;
    Ok(fuse(topo, cylinder, pad)?)
}
