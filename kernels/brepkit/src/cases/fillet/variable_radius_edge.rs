//! A cube of side 10 from the origin, its edge from (10, 10, 0) to (10, 10, 10) rounded with a
//! radius growing linearly along it, 1 at z = 0 to 2 at z = 10: brepkit's variable fillet with a
//! linear radius law, its start and end taken from the ends of the edge as the kernel stores it.
use brepkit_operations::blend_ops::{all_blended, fillet_v2_variable};
use brepkit_operations::fillet::FilletRadiusLaw;
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{edges_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cube = make_box(topo, 10.0, 10.0, 10.0)?;
    let edge = edges_where(topo, cube, |q| near(q.x(), 10.0) && near(q.y(), 10.0))?;
    let [edge] = edge[..] else {
        return Err("the cube has no single edge at x = 10, y = 10".into());
    };
    // r = 1 + z/10 from the edge's start to its end
    let e = topo.edge(edge)?;
    let (z0, z1) = (
        topo.vertex(e.start())?.point().z(),
        topo.vertex(e.end())?.point().z(),
    );
    let law = FilletRadiusLaw::Linear {
        start: 1.0 + z0 / 10.0,
        end: 1.0 + z1 / 10.0,
    };
    Ok(all_blended(fillet_v2_variable(
        topo,
        cube,
        vec![(edge, (&law).into())],
    )?)?)
}
