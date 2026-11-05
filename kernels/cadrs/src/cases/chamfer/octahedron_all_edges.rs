//! The octahedron |x| + |y| + |z| ≤ 10, all twelve edges chamfered with the same distance √6 on
//! both faces, measured across the face square to the edge (the chamfer of the edge from
//! (10, 0, 0) to (0, 10, 0) the plane x + y = 8), the four chamfers at each corner closed by a
//! flat square: the octahedron cut by the twelve chamfer planes and by the six corner planes
//! |x| ≤ 6, |y| ≤ 6 and |z| ≤ 6.
//!
//! The octahedron is two square pyramids, each the square through its corners in z = 0 lofted
//! to a point, united; its twelve edges are chamfered in one Chamfer feature, equal distance √6.
//! cadrs's chamfer has no corner face: it runs the four chamfers at a corner on until they meet
//! in a point (the miter of `chamfer/octahedron-all-edges-mitered`). So each corner is then cut
//! by its plane, as the case composes the solid: a disc sketched on the plane x = 6 (and the
//! others), extruded outward past the miter's point and cut away.
use crate::bench::{all_edges, p3, v3};
use crate::shared::chamfer::{chamfer, cut_corner, octahedron};
use crate::Refusal;
use cadrs_kernel::{BodyId, ChamferSpec, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let o = octahedron(k, 10.0)?;
    let edges = all_edges(k, o)?;
    let mut body = chamfer(k, o, &edges, ChamferSpec::EqualDistance(6f64.sqrt()))?;
    let axes = [v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), v3(0.0, 0.0, 1.0)];
    for axis in axes {
        for sign in [1.0, -1.0] {
            let out = axis * sign;
            body = cut_corner(k, body, p3(0.0, 0.0, 0.0) + out * 6.0, out, 4.0, 4.0)?;
        }
    }
    Ok(body)
}
