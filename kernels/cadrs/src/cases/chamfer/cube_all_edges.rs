//! A cube of side 10 between the origin and (10, 10, 10), all twelve edges chamfered with the
//! same distance 2 on both faces, the three chamfers at each corner closed by a flat triangle:
//! the cube cut by the twelve chamfer planes (x + y ≤ 18 and the others) and by the eight corner
//! planes (x + y + z ≤ 26 at the corner (10, 10, 10) and the others).
//!
//! The cube is a square extruded, and its twelve edges are chamfered in one Chamfer feature,
//! equal distance 2. cadrs's chamfer has no corner face: it runs the three chamfers at a corner
//! on until they meet in a point (the miter of `chamfer/cube-all-edges-mitered`). So each corner
//! is then cut by its plane, as the case composes the solid: a disc sketched on the plane through
//! the three square corners nearest the vertex, extruded outward past the miter's point and cut
//! away.
use crate::bench::{all_edges, block, p3, v3};
use crate::shared::chamfer::{chamfer, cut_corner};
use crate::Refusal;
use cadrs_kernel::{BodyId, ChamferSpec, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cube = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let edges = all_edges(k, cube)?;
    let mut body = chamfer(k, cube, &edges, ChamferSpec::EqualDistance(2.0))?;
    for sx in [-1.0, 1.0] {
        for sy in [-1.0, 1.0] {
            for sz in [-1.0, 1.0] {
                // the corner's plane through the three square corners nearest it, about the
                // middle of their triangle: at the corner (10, 10, 10) the triangle through
                // (8, 8, 10), (8, 10, 8) and (10, 8, 8), whose middle is (26/3, 26/3, 26/3)
                let out = v3(sx, sy, sz);
                let middle = p3(5.0, 5.0, 5.0) + out * (3.0 + 2.0 / 3.0);
                body = cut_corner(k, body, middle, out, 3.0, 4.0)?;
            }
        }
    }
    Ok(body)
}
