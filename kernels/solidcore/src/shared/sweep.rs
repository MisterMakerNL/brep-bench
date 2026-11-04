//! What the solidcore implementations of the sweep cases share.
use crate::bench::circle;
use crate::Refusal;
use solidcore::curve::arc::ArcDefinition;
use solidcore::curve::curve::Curve as KCurve;
use solidcore::curve::line::Line;
use solidcore::curve::path::PathPiece;
use solidcore::sweep::api_brep::create_sweep_along_planar_path;
use solidcore::{Point3, Vec3};
use solidcore_ops::sweep::{screw_sweep, Curve};
use solidcore_ops::{Body, Result};
use std::f64::consts::TAU;

/// How near the screw sweep's walls keep to their screw surfaces.
pub const SCREW_TOLERANCE: f64 = 1e-4;

/// The disc of radius `r` square to the right-handed helix about the z axis through `start`
/// (on the xz plane's +x side, `radius` from the axis) rising `pitch` per turn, swept by the
/// helix's screw for `turns` turns: a pipe about the helix. The screw carries the helix onto
/// itself and its tangent with it, so the section stays square to the path all the way.
pub fn helix_pipe(start: [f64; 3], radius: f64, pitch: f64, turns: f64, r: f64) -> Result<Body> {
    // the helix's tangent at the start: a turn's circumference along +y, a pitch up z
    let (along, up) = (TAU * radius, pitch);
    let length = (along * along + up * up).sqrt();
    let tangent = [0.0, along / length, up / length];
    let section = circle(start, tangent, [1.0, 0.0, 0.0], r);
    screw_sweep(
        &[section],
        [0.0, 0.0, start[2]],
        [0.0, 0.0, 1.0],
        pitch,
        turns,
        None,
        SCREW_TOLERANCE,
    )
}

/// The pipe of the helical-pipe cases: radius 1 about the helix of radius 10 about the z axis
/// rising 5 per turn, from (10, 0, 0), right-handed, three whole turns, its ends flat.
pub fn helical_pipe() -> Result<Body> {
    helix_pipe([10.0, 0.0, 0.0], 10.0, 5.0, 3.0, 1.0)
}

/// An ops curve as the kernel's: a line its segment, an arc its circle's arc.
fn kernel_curve(k: &solidcore::kernel::Kernel, c: &Curve) -> std::result::Result<KCurve, Refusal> {
    let p = |a: [f64; 3]| Point3::new(a[0], a[1], a[2]);
    Ok(match c {
        Curve::Line { start, end } => {
            KCurve::from(Line::create_line_segment(3, p(*start), p(*end), None)?)
        }
        Curve::Arc { center, axis, reference, radius, start, end } => {
            let v = |a: [f64; 3]| Vec3::new(a[0], a[1], a[2]);
            let arc = ArcDefinition {
                centre: p(*center),
                axis: v(*axis),
                reference: v(*reference),
                radius: *radius,
                start: *start,
                end: *end,
            };
            PathPiece::of_arc(k, &arc)?.curve().clone()
        }
        Curve::Nurbs { .. } => return Err("a spline is not taken here".into()),
    })
}

/// The planar `profile`, standing square to the planar `path` of lines and arcs at its start,
/// swept along it with each run mitred to the next where the path turns sharply, in the plane
/// that bisects the corner: the kernel's sweep along a planar path
/// (`SmApiCreateSweepAlongPlanarPath`, one capped sweep per path curve, mitred at the tangent
/// breaks, the pieces united), the ends of an open path capped. The kernel carries the profile
/// by the middle of its box, which it sets on the path's start. Ops has no call for it (its
/// `sweep_placed` refuses a path with a corner, `status 1001`), so it is called on the kernel,
/// and the body brought back through its record.
pub fn mitred(profile: &[Curve], path: &[Curve]) -> std::result::Result<Body, Refusal> {
    let mut k = crate::shared::swept::session_kernel();
    let profile =
        profile.iter().map(|c| kernel_curve(&k, c)).collect::<std::result::Result<Vec<_>, _>>()?;
    let path =
        path.iter().map(|c| kernel_curve(&k, c)).collect::<std::result::Result<Vec<_>, _>>()?;
    let profile: Vec<&KCurve> = profile.iter().collect();
    let path: Vec<&KCurve> = path.iter().collect();
    let mut swept = create_sweep_along_planar_path(&mut k, &profile, &path, true, 3)?;
    let (record, _) = solidcore_convert::brep::to_record(&mut k, &mut swept)
        .map_err(|e| format!("the swept body's record was not made: {e:?}"))?;
    Ok(Body::from_record(&record, true)?.0)
}
