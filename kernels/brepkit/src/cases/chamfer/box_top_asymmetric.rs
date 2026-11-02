//! A box from the origin to (20, 10, 6), the four edges around its top face chamfered in one
//! chamfer with two distances: 1 across the top face, each edge's reference face, and 2 down the
//! side face beside it. Neighbouring chamfers run on until they meet.
use brepkit_operations::blend_ops::{all_blended, chamfer_v2_on};
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{edges_where, faces_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 20.0, 10.0, 6.0)?;
    let top = faces_where(topo, block, |q| near(q.z(), 6.0))?;
    let [top] = top[..] else {
        return Err("the box has no single face at z = 6".into());
    };
    let rim = edges_where(topo, block, |q| near(q.z(), 6.0))?;
    let on_top: Vec<_> = rim.into_iter().map(|e| (e, top)).collect();
    Ok(all_blended(chamfer_v2_on(topo, block, &on_top, 1.0, 2.0)?)?)
}
