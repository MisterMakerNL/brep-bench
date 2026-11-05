//! What the cadrs implementations of the swept cases share: the rod several of them groove, and
//! the solid a torus fills as it slides along the z axis.
//!
//! cadrs has no sweep of a solid (its `sweep_body` carries points, wires and sheets one kind
//! higher, never a solid), so a swept solid is built from what it is by elementary geometry,
//! as the cases' descriptions give it.
use crate::bench::{arc, chain, cylinder, extrude, line, p2, p3, sketch, torus, v3, xy, P3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

/// The rod: the cylinder of radius 5 about the z axis from z = 0 to z = 20.
pub fn rod(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 5.0, 20.0)
}

/// What the torus about the axis through `center` parallel to the y axis, of radii `major` and
/// `minor`, fills as it slides without turning `rise` up the z axis: the torus where it starts
/// and where it ends, and between them the region it covers seen along the slide (its shadow on
/// the plane z = `center.z`, the stadium of half-width `minor` about the segment from
/// `center.x − major` to `center.x + major` along x) extruded `rise` up. Every point the torus
/// passes lies in one of them.
pub fn torus_slid(
    k: &mut DefaultKernel,
    center: P3,
    major: f64,
    minor: f64,
    rise: f64,
) -> Result<Vec<BodyId>, Refusal> {
    let start = torus(k, center, v3(0.0, 1.0, 0.0), major, minor)?;
    let end = torus(k, center + v3(0.0, 0.0, rise), v3(0.0, 1.0, 0.0), major, minor)?;
    let (x0, x1, y, r) = (center.x - major, center.x + major, center.y, minor);
    let stadium = chain(vec![
        line(p2(x0, y - r), p2(x1, y - r)),
        arc(p2(x1, y), r, -90.0, 180.0),
        line(p2(x1, y + r), p2(x0, y + r)),
        arc(p2(x0, y), r, 90.0, 180.0),
    ]);
    let shadow = extrude(k, &sketch(xy(center.z), vec![stadium]), rise)?;
    Ok(vec![start, shadow, end])
}
