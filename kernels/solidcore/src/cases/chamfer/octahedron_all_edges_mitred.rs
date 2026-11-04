//! The octahedron |x| + |y| + |z| ≤ 10, all twelve edges chamfered with distance √6 on both
//! faces (the chamfer of the edge from (10, 0, 0) to (0, 10, 0) the plane x + y = 8), one after
//! another, each its own chamfer ending on the faces at its ends: the four chamfers at each
//! corner mitre, meeting in a point.
use crate::shared::chamfer::{chamfer_one_by_one, octahedron};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // the middle of each edge, which each chamfer before it leaves on it
    let middles = [
        [5.0, 5.0, 0.0],
        [-5.0, 5.0, 0.0],
        [-5.0, -5.0, 0.0],
        [5.0, -5.0, 0.0],
        [5.0, 0.0, 5.0],
        [0.0, 5.0, 5.0],
        [-5.0, 0.0, 5.0],
        [0.0, -5.0, 5.0],
        [5.0, 0.0, -5.0],
        [0.0, 5.0, -5.0],
        [-5.0, 0.0, -5.0],
        [0.0, -5.0, -5.0],
    ];
    chamfer_one_by_one(octahedron()?, &middles, 6f64.sqrt())
}
