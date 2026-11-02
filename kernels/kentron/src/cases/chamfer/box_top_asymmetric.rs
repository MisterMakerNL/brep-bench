//! A box from the origin to (20, 10, 6), the four edges around its top face z = 6 each chamfered
//! with distance 1 across the top and 2 down the side: the box cut by those four planes, its top
//! the rectangle from (1, 1, 6) to (19, 9, 6).
use crate::bench::{face_through, p};
use crate::Refusal;
use kentron::blend::{self, ChamferSize, ChamferedEdge};
use kentron::primitives::Cuboid;
use kentron::topology::{Model, NodeKind, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 10.0, 6.0)).build(m)?;
    let top = face_through(m, block, p(10.0, 5.0, 6.0))?;
    // The top's four edges, each with its first distance measured across the top.
    let edges: Vec<ChamferedEdge> = m
        .unique_subshapes(top, NodeKind::Edge)
        .into_iter()
        .map(|edge| ChamferedEdge { edge, size: ChamferSize::Distances(1.0, 2.0), face: Some(top) })
        .collect();
    Ok(blend::chamfer_edges(m, block, &edges)?)
}
