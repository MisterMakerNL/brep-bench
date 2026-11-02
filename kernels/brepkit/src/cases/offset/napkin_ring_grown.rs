//! A napkin ring, the ball of radius 5 about the origin bored through along the z axis by a hole
//! of radius 3, grown by 1 with its rims rounded.
//!
//! brepkit's rounded offset (`offset_solid_arc_v2`) does not build its arc joints yet, so the
//! ring is offset with sharp joints (its sphere grown to radius 6, its bore narrowed to radius 2)
//! and the grown ring's two rims are then rounded to radius 1: each round lies 1 from the moved
//! sphere and bore, on the tube of radius 1 about the ring's own rim, the circle of radius 3 at
//! z = ±4.
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_operations::primitives::{make_cylinder, make_sphere};
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{cut, edges_where, moved, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let ball = make_sphere(topo, 5.0, 64)?;
    // the bore runs past both ends
    let bore = make_cylinder(topo, 3.0, 12.0)?;
    let bore = moved(topo, bore, 0.0, 0.0, -6.0)?;
    let ring = cut(topo, ball, bore)?;
    let grown = offset_solid_v2(topo, ring, 1.0)?;
    // the rims: where the sphere of radius 6 meets the bore of radius 2
    let rims = edges_where(topo, grown, |q| {
        near(q.x().hypot(q.y()), 2.0) && near(q.x() * q.x() + q.y() * q.y() + q.z() * q.z(), 36.0)
    })?;
    if rims.is_empty() {
        return Err(
            "brepkit's offset of the ring has no rim where the grown sphere meets the grown bore \
                    to round (the offset solid it returns fails brepkit's own validity check)"
                .into(),
        );
    }
    Ok(all_blended(fillet_v2(topo, grown, &rims, 1.0)?)?)
}
