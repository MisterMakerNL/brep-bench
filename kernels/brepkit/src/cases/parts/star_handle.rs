//! A four-armed star handle: the outline in the xz plane of four tip arcs (radius 4, centred 33.1
//! out on the axes) and four fillet arcs (radius 3, centred 15.9 out on the diagonals), each tip
//! joined to the fillets beside it by the lines tangent to both that pass between them, drawn
//! from y = 0 to y = 12 with a 2° draft, its section at height h moved h·tan 2° inward.
//!
//! brepkit's draft turns planar faces only, and refuses a face whose vertices meet a curved one,
//! so the extruded outline's arcs cannot be drafted. Instead the drafted sides are lofted: the
//! outline is drawn at y = 0 and again at y = 12 as the description gives it there (each side
//! moved in parallel to itself, the tip arcs shrunk and the fillet arcs grown about their
//! centres), and brepkit's loft joins each edge to its fellow by a ruled face, which keeps a
//! line to a parallel line as a plane and an arc to an arc about the same axis as a cone.
use brepkit_operations::loft::loft;
use brepkit_topology::face::FaceId;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

use crate::solids::contact::{outline_face, Run};
use crate::solids::{p, v};
use crate::Refusal;

const TIP_DISTANCE: f64 = 33.1;
const TIP_RADIUS: f64 = 4.0;
const FILLET_DISTANCE: f64 = 15.9;
const FILLET_RADIUS: f64 = 3.0;
const HEIGHT: f64 = 12.0;
const DRAFT_DEGREES: f64 = 2.0;

/// `[x, z]` turned about the origin by `angle`, from +x towards +z.
fn turned([x, z]: [f64; 2], angle: f64) -> [f64; 2] {
    let (s, c) = angle.sin_cos();
    [c * x - s * z, s * x + c * z]
}

/// The outline in the plane `y`, its tip arcs of radius `tip` and its fillet arcs of radius
/// `fillet`, as a face.
fn outline(topo: &mut Topology, y: f64, tip: f64, fillet: f64) -> Result<FaceId, Refusal> {
    let at = |[x, z]: [f64; 2]| p(x, y, z);
    let tip_centre = [TIP_DISTANCE, 0.0];
    let fillet_centre = turned([FILLET_DISTANCE, 0.0], FRAC_PI_4);
    // The side from the tip on +x to the fillet circle at 45°, tangent to both and between them:
    // its unit normal n has the tip's centre `tip` to one side and the fillet's `fillet` to the
    // other, so n · (tip centre − fillet centre) = tip + fillet; of the two such lines, n the
    // line of centres turned clockwise is the one bounding the arm.
    let (dx, dz) = (
        tip_centre[0] - fillet_centre[0],
        tip_centre[1] - fillet_centre[1],
    );
    let gap = dx.hypot(dz);
    let n = turned([dx / gap, dz / gap], -((tip + fillet) / gap).acos());
    let on_tip = [tip_centre[0] - tip * n[0], tip_centre[1] - tip * n[1]];
    let on_fillet = [
        fillet_centre[0] + fillet * n[0],
        fillet_centre[1] + fillet * n[1],
    ];
    // The arm's other side, mirrored across its centre line.
    let mirror = |[x, z]: [f64; 2]| [x, -z];
    // Turning from +x towards +z is counter-clockwise about −y: the tip arcs run that way, out
    // round the tip, and the fillet arcs the other way, in round the fillet circle's near side.
    let mut corners = Vec::new();
    for arm in 0..4 {
        let a = FRAC_PI_2 * f64::from(arm);
        let tip_arc = Run::Arc {
            centre: at(turned(tip_centre, a)),
            axis: v(0.0, -1.0, 0.0),
        };
        let fillet_arc = Run::Arc {
            centre: at(turned(fillet_centre, a)),
            axis: v(0.0, 1.0, 0.0),
        };
        corners.push((at(turned(mirror(on_tip), a)), tip_arc));
        corners.push((at(turned(on_tip, a)), Run::Line));
        corners.push((at(turned(on_fillet, a)), fillet_arc));
        corners.push((at(turned(mirror(on_fillet), a + FRAC_PI_2)), Run::Line));
    }
    outline_face(topo, &corners)
}

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let inset = HEIGHT * DRAFT_DEGREES.to_radians().tan();
    let bottom = outline(topo, 0.0, TIP_RADIUS, FILLET_RADIUS)?;
    let top = outline(topo, HEIGHT, TIP_RADIUS - inset, FILLET_RADIUS + inset)?;
    Ok(loft(topo, &[bottom, top])?)
}
