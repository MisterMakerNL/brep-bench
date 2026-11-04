//! The octahedron |x| + |y| + |z| ≤ 10, all twelve edges chamfered with distance √6 on both
//! faces (the chamfer of the edge from (10, 0, 0) to (0, 10, 0) the plane x + y = 8), the four
//! chamfers at each corner closed by a flat square.
use crate::bench::{all_edges, reloaded};
use crate::shared::chamfer::{chamfer_equal, octahedron};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let (solid, order) = reloaded(&octahedron()?)?;
    chamfer_equal(&solid, &all_edges(&order), 6f64.sqrt())?;
    Ok(solid)
}
