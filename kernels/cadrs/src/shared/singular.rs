//! What the cadrs implementations of the singular cases share: cutting away everything on one
//! side of a plane.
use crate::bench::{cut, extrude, rectangle, sketch, square_to, P3, V3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

/// `body` less everything on the side `normal` points to of the plane through `point`: cut
/// with a block sketched as a square of side 60 on that plane, centred on `point`, and extruded
/// 30 along `normal` (the bodies cut here lie within 20 of the origin).
pub fn cut_beyond(
    k: &mut DefaultKernel,
    body: BodyId,
    point: P3,
    normal: V3,
) -> Result<BodyId, Refusal> {
    let s = sketch(square_to(point, normal), vec![rectangle(-30.0, -30.0, 30.0, 30.0)]);
    let tool = extrude(k, &s, 30.0)?;
    cut(k, body, tool)
}

/// The ball of `radius` about `center` with its poles on the axis through `center` along `dir`:
/// the half disc beside that axis, sketched on a plane through it, revolved a whole turn about
/// it (cadrs has no sphere primitive).
pub fn ball_about(
    k: &mut DefaultKernel,
    center: P3,
    dir: V3,
    radius: f64,
) -> Result<BodyId, Refusal> {
    use crate::bench::{arc, axis, chain, line, p2, revolve, through_axis};
    let half = chain(vec![
        arc(p2(0.0, 0.0), radius, -90.0, 180.0),
        line(p2(0.0, radius), p2(0.0, -radius)),
    ]);
    let s = sketch(through_axis(center, dir), vec![half]);
    revolve(k, &s, axis(center, dir), None)
}
