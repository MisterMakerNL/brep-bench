//! Helpers shared by the draft cases: the faces a case drafts, picked as a user picks them, and the
//! rounded rectangle the rounded cases are extruded from. Every case drafts with the kernel's
//! `draft_faces` (or `draft_faces_parted`, for a parting line).
#![allow(unused_imports)]
use super::*;
use brep_kernel::{make_arc, make_line, resolve_face_by_point, BrepSolid, KernelRefusal, NurbsCurve};

/// The faces through `points`, one point on each.
pub fn faces_at(solid: &BrepSolid, points: &[Vec3]) -> Result<Vec<u64>, KernelRefusal> {
    points.iter().map(|&point| resolve_face_by_point(solid, point)).collect()
}

/// The faces that run along z (each face's normal is square to z) whose corners' centroid passes
/// `keep`: the walls of a block or of a pocket, picked as a user picks them, by region.
pub fn walls(solid: &BrepSolid, keep: impl Fn(Vec3) -> bool) -> Result<Vec<u64>, String> {
    let vertex = |id: u64| solid.vertices.iter().find(|v| v.id == id).map(|v| v.point);
    let mut ids = Vec::new();
    for shell in &solid.shells {
        for face in &shell.faces {
            let [u0, u1] = face.surface.domain_u()?;
            let [v0, v1] = face.surface.domain_v()?;
            let normal = face.surface.normal((u0 + u1) * 0.5, (v0 + v1) * 0.5)?;
            if normal.z.abs() > 1e-9 {
                continue;
            }
            let (mut sum, mut count) = (p(0.0, 0.0, 0.0), 0.0);
            for coedge in face.loops.iter().flat_map(|l| l.coedges.iter()) {
                let edge = solid.edges.iter().find(|e| e.id == coedge.edge_id).ok_or("missing edge")?;
                sum = sum.add(vertex(edge.start_vertex_id).ok_or("missing vertex")?);
                count += 1.0;
            }
            if count > 0.0 && keep(sum.scale(1.0 / count)) {
                ids.push(face.id);
            }
        }
    }
    Ok(ids)
}

/// The rectangle from (x0, y0) to (x1, y1) at height z, its corners rounded to `r`, as lines and
/// quarter arcs, counter-clockwise from +z.
pub fn rounded_rectangle(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    r: f64,
    z: f64,
) -> Result<Vec<NurbsCurve>, String> {
    let (ex, ey) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    let half_pi = std::f64::consts::FRAC_PI_2;
    Ok(vec![
        make_line(p(x0 + r, y0, z), p(x1 - r, y0, z))?,
        make_arc(p(x1 - r, y0 + r, z), ex, ey, r, -half_pi, 0.0)?,
        make_line(p(x1, y0 + r, z), p(x1, y1 - r, z))?,
        make_arc(p(x1 - r, y1 - r, z), ex, ey, r, 0.0, half_pi)?,
        make_line(p(x1 - r, y1, z), p(x0 + r, y1, z))?,
        make_arc(p(x0 + r, y1 - r, z), ex, ey, r, half_pi, 2.0 * half_pi)?,
        make_line(p(x0, y1 - r, z), p(x0, y0 + r, z))?,
        make_arc(p(x0 + r, y0 + r, z), ex, ey, r, 2.0 * half_pi, 3.0 * half_pi)?,
    ])
}
