//! A plate from (−20, −20, 0) to (20, 20, 4) fused with a boss, the cylinder of radius 5 on the z
//! axis from z = 4 to 8; both of the boss's circles, its rim at z = 8 and its root at z = 4, rounded
//! to radius 2 in one fillet, so the two rounds meet at z = 6 and the boss's side is used up.
use brepkit_operations::primitives::{make_box, make_cylinder};
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::rounded;
use crate::solids::{edges_where, fuse, moved, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let plate = make_box(topo, 40.0, 40.0, 4.0)?;
    let plate = moved(topo, plate, -20.0, -20.0, 0.0)?;
    let boss = make_cylinder(topo, 5.0, 4.0)?;
    let boss = moved(topo, boss, 0.0, 0.0, 4.0)?;
    let part = fuse(topo, plate, boss)?;
    // the boss's two circles: every point 5 from the z axis, at z = 4 or z = 8
    let circles = edges_where(topo, part, |q| {
        (q.x().hypot(q.y()) - 5.0).abs() < 1e-7 && (near(q.z(), 4.0) || near(q.z(), 8.0))
    })?;
    Ok(rounded(topo, part, &circles, 2.0)?)
}
