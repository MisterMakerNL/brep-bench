//! What the sweep, loft and part cases share besides [`super`]: planes offset from the origin's,
//! 3-D sketches of points in space, and a sweep or loft written briefly.
use crate::shared::n;
use crate::{Bench, Refusal};
use geop_core_math::{primitives::FrameAxis, vector::Vector3};
use geop_core_program::element::ElementReference;
use geop_ops_extrude_revolve::{LoftArgs, Output, PathRef, SweepArgs};
use geop_ops_model::Combine;
use geop_ops_model::datums::{datum, datums, origin_plane};
use geop_ops_model::selection::Selected;
use geop_ops_model::sketch::{SketchPlane, sketch, whole_sketch};
use geop_ops_model::sketch3d::{Sketch3d, sketch3d_path};
use geop_ops_reference_geometry::{AddDatumArgs, Construction};
use geop_ops_sketch3d::AddSketch3dArgs;
use geop_lib_sweep::Orientation;

/// Adds the datum plane `id`, the origin's plane normal to `normal` moved `distance` along its
/// normal, and gives it as a plane to sketch on, drawn in the same coordinates as the origin's.
pub fn offset_plane(
    b: &mut Bench,
    id: &str,
    normal: FrameAxis,
    distance: f64,
) -> Result<SketchPlane, Refusal> {
    b.step(
        id,
        AddDatumArgs {
            selection: vec![Selected::Datum(datum(origin_plane(normal).name()))],
            construction: Construction::Offset { distance: distance.into() },
        },
    )?;
    Ok(SketchPlane::Datum(ElementReference::new(datums(), id)))
}

/// A point in space, for a 3-D sketch.
pub fn at(p: [f64; 3]) -> Vector3 {
    Vector3::from_array(p.map(n))
}

/// Adds the 3-D sketch `id`, `sketch`.
pub fn sketch3d(b: &mut Bench, id: &str, sketch: Sketch3d) -> Result<(), Refusal> {
    b.step(id, AddSketch3dArgs { sketch, references: Vec::new() })
}

/// The 3-D sketch `id`'s one chain, as a path to sweep along.
pub fn path3d(id: &str) -> PathRef {
    PathRef::Sketch3d(sketch3d_path(id))
}

/// The sketch `id`'s one chain or loop, as a path to sweep along.
pub fn path2d(id: &str) -> PathRef {
    PathRef::Sketch(ElementReference::whole(sketch(id)))
}

/// Sweeps the sketch `profile` along `path`, turning with it, as the step `id`, kept or combined
/// as `combine` says.
pub fn sweep(
    b: &mut Bench,
    id: &str,
    profile: &str,
    path: PathRef,
    combine: Combine,
) -> Result<(), Refusal> {
    b.step(
        id,
        SweepArgs {
            profile: whole_sketch(profile),
            path: Some(path),
            orientation: Orientation::FollowPath,
            twist: 0.0,
            end_scale: 1.0,
            rails: Vec::new(),
            output: Output::Solid(combine),
        },
    )
}

/// Lofts through the sketches `profiles`, in order, guided by `guides` (if any), as the step `id`,
/// a new body.
pub fn loft(b: &mut Bench, id: &str, profiles: &[&str], guides: Vec<PathRef>) -> Result<(), Refusal> {
    b.step(
        id,
        LoftArgs {
            profiles: profiles.iter().map(|p| whole_sketch(p)).collect(),
            matches: Vec::new(),
            guides,
            output: Output::Solid(Combine::NewBody),
        },
    )
}

/// Adds the datum point `id`, `p` from the origin.
pub fn point_datum(b: &mut Bench, id: &str, p: [f64; 3]) -> Result<(), Refusal> {
    b.step(
        id,
        AddDatumArgs {
            selection: vec![Selected::Datum(datum("origin"))],
            construction: Construction::Point { x: p[0].into(), y: p[1].into(), z: p[2].into() },
        },
    )
}

/// Adds the datum plane `id` through `at`, square to `direction` — the plane a section is drawn
/// in where a path through `at` heads along `direction` — built as a designer builds it: the
/// point, a second one ahead along the direction, the axis through both and the plane through the
/// first square to it. Sketched on, its origin is `at`; its first axis is the world axis most
/// nearly square to `direction`, made square to it (so z, for a direction in the xy plane), its
/// second `direction × first`.
pub fn plane_square_to(
    b: &mut Bench,
    id: &str,
    at: [f64; 3],
    direction: [f64; 3],
) -> Result<SketchPlane, Refusal> {
    let (here, ahead, axis) = (format!("{id}_at"), format!("{id}_ahead"), format!("{id}_axis"));
    point_datum(b, &here, at)?;
    point_datum(b, &ahead, [0, 1, 2].map(|k| at[k] + direction[k]))?;
    b.step(
        &axis,
        AddDatumArgs {
            selection: vec![Selected::Datum(datum(&here)), Selected::Datum(datum(&ahead))],
            construction: Construction::TwoPoints {},
        },
    )?;
    b.step(
        id,
        AddDatumArgs {
            selection: vec![Selected::Datum(datum(&axis)), Selected::Datum(datum(&here))],
            construction: Construction::NormalToLine {},
        },
    )?;
    Ok(SketchPlane::Datum(ElementReference::new(datums(), id)))
}

/// Where the point `p` is drawn on the plane [`plane_square_to`] makes through `at` square to
/// `direction`: its coordinates along that plane's two axes, from `at`.
pub fn drawn_at(at: [f64; 3], direction: [f64; 3], p: [f64; 3]) -> [f64; 2] {
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let length = dot(direction, direction).sqrt();
    let w = direction.map(|c| c / length);
    // The world axis most nearly square to the normal, made square to it.
    let least = (0..3).fold(0, |k, j| if w[j].abs() < w[k].abs() { j } else { k });
    let mut u = [0.0; 3];
    u[least] = 1.0;
    let along = w[least];
    u = [0, 1, 2].map(|k| u[k] - w[k] * along);
    let size = dot(u, u).sqrt();
    u = u.map(|c| c / size);
    let v = [w[1] * u[2] - w[2] * u[1], w[2] * u[0] - w[0] * u[2], w[0] * u[1] - w[1] * u[0]];
    let d = [0, 1, 2].map(|k| p[k] - at[k]);
    [dot(d, u), dot(d, v)]
}

/// Turns the ball of `radius` about `centre` from a half disc on a plane parallel to the xz plane
/// through it, about the half disc's upright side, as the step `id`, kept or combined as
/// `combine` says. geop has no ball of its own.
pub fn ball(
    b: &mut Bench,
    id: &str,
    centre: [f64; 3],
    radius: f64,
    combine: Combine,
) -> Result<(), Refusal> {
    use crate::shared::{Drawing, revolve, sketch, sketch_axis, zx, zx_at};
    let plane = match centre[1] {
        0.0 => zx(),
        y => offset_plane(b, &format!("{id}_plane"), FrameAxis::Y, y)?,
    };
    let half = format!("{id}_half");
    let [x, _, z] = centre;
    let mut disc = Drawing::new();
    let below = disc.point(zx_at(x, z - radius));
    let above = disc.point(zx_at(x, z + radius));
    let upright = disc.line(below, above);
    disc.arc(above, below, 180.0);
    sketch(b, &half, plane, disc)?;
    revolve(b, id, &half, sketch_axis(&half, upright), 360.0, combine)
}

/// The slot between the circles of `radius` about (5.5, y) and (17.5, y) and the lines tangent
/// to both: the shadow along z of a torus with its axis along y, its tube of `radius` on a circle
/// of radius 6 about (11.5, y).
pub fn slot(y: f64, radius: f64) -> crate::shared::Drawing {
    use crate::shared::Drawing;
    let mut slot = Drawing::new();
    let p = [[5.5, y + radius], [5.5, y - radius], [17.5, y - radius], [17.5, y + radius]]
        .map(|q| slot.point(q));
    slot.arc(p[0], p[1], 180.0);
    slot.line(p[1], p[2]);
    slot.arc(p[2], p[3], 180.0);
    slot.line(p[3], p[0]);
    slot
}
