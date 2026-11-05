//! Helpers shared by the draft cases. The kernel has no draft of a face: a planar side is drafted
//! by turning it, with the kernel's face rotation, about its neutral line (where it crosses the
//! neutral plane), which re-intersects the faces beside it; a curved side by the kernel's drafted
//! extrusion of its section.
#![allow(unused_imports)]
use super::*;
use brep_kernel::{
    make_arc, make_line, resolve_face_by_point, rotate_faces, BrepSolid, KernelRefusal,
    NurbsCurve,
};

/// `solid` with the planar side through `on_side`, whose outward normal is the horizontal `normal`,
/// drafted by `angle` (radians) for the pull direction +z about the neutral plane z = `neutral`:
/// the side turns about the horizontal line where it crosses that plane, leaning `angle` from +z,
/// inward (into the material) as it rises for a positive angle, outward for a negative one.
pub fn draft_side(
    solid: &BrepSolid,
    on_side: Vec3,
    normal: Vec3,
    neutral: f64,
    angle: f64,
) -> Result<BrepSolid, KernelRefusal> {
    let face = resolve_face_by_point(solid, on_side)?;
    // Turning about n × z by a positive angle carries +z toward −n, into the material.
    let hinge = normal.cross(p(0.0, 0.0, 1.0));
    let on_hinge = p(on_side.x, on_side.y, neutral);
    rotate_faces(solid, &[face], on_hinge, hinge, angle)
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

/// The circle of `radius` about the vertical line through `center`, in the plane z = center.z, as
/// two half arcs: the kernel's drafted extrusion takes a loop of at least two curves.
pub fn circle_halves(center: Vec3, radius: f64) -> Result<Vec<NurbsCurve>, String> {
    let (ex, ey) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    let pi = std::f64::consts::PI;
    Ok(vec![make_arc(center, ex, ey, radius, 0.0, pi)?, make_arc(center, ex, ey, radius, pi, 2.0 * pi)?])
}
