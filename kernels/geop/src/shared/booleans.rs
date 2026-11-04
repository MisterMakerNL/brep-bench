//! What the boolean, tangent and singular cases share: the solids they combine, each made the
//! way a geop user makes it (geop has no primitive solids: a block or a drum is extruded, a
//! ball, a cone or a torus is turned from its section), and the boolean that combines two.
use crate::shared::{Drawing, made, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::{DatumComponent, FrameAxis};
use geop_ops_boolean::BooleanArgs;
use geop_ops_extrude_revolve::{Extents, ExtrudeArgs, Output};
use geop_ops_model::datums::{ORIGIN, component_name, datum};
use geop_ops_model::selection::Selected;
use geop_ops_model::sketch::{SketchPlane, whole_sketch};
use geop_ops_model::{BooleanOp, Combine, Extent, solid};
use geop_ops_reference_geometry::{AddDatumArgs, Construction};

/// Extrudes the whole sketch `sketch` from `from` to `to` along its plane's normal (`from` ≤ 0 ≤
/// `to`: the sketch's plane lies between the two ends), as the step `id`, kept or combined as
/// `combine` says.
pub fn extrude_between(
    b: &mut Bench,
    id: &str,
    sketch: &str,
    from: f64,
    to: f64,
    combine: Combine,
) -> Result<(), Refusal> {
    b.step(
        id,
        ExtrudeArgs {
            sketch: whole_sketch(sketch),
            extent: Extents { side2: Some(Extent::blind(-from)), ..Extents::blind(to) },
            output: Output::Solid(combine),
            draft: 0.0,
        },
    )
}

/// The boolean `op` of the solids `a` and `b`, as the step `id`: the result is `boolean(id)`.
pub fn boolean(b: &mut Bench, id: &str, a: &str, other: &str, op: BooleanOp) -> Result<(), Refusal> {
    b.step(id, BooleanArgs { a: solid(a), b: solid(other), op })
}

/// The datum plane `id`, the origin's plane normal to `normal` moved `distance` along it.
pub fn offset_plane(
    b: &mut Bench,
    id: &str,
    normal: FrameAxis,
    distance: f64,
) -> Result<SketchPlane, Refusal> {
    b.step(
        id,
        AddDatumArgs {
            selection: vec![Selected::Datum(datum(component_name(
                ORIGIN,
                DatumComponent::Plane(normal),
            )))],
            construction: Construction::Offset { distance: distance.into() },
        },
    )?;
    Ok(SketchPlane::Datum(datum(id)))
}

/// A ball of `radius` about `center`, as the step `id` (the solid `revolve(id)`): a half disc
/// drawn in the plane through its centre parallel to the xz plane (the xz plane itself, or a
/// datum plane offset from it), bounded by the vertical through the centre, turned a full turn
/// about that line. Its poles are on that vertical, above and below the centre.
pub fn ball(b: &mut Bench, id: &str, center: [f64; 3], radius: f64) -> Result<String, Refusal> {
    let [x, y, z] = center;
    let plane = if y == 0.0 { zx() } else { offset_plane(b, &format!("{id}_plane"), FrameAxis::Y, y)? };
    let mut half = Drawing::new();
    let bottom = half.point(zx_at(x, z - radius));
    let top = half.point(zx_at(x, z + radius));
    let on_axis = half.line(bottom, top);
    // From the top, counter-clockwise in the sketch (whose second axis is −z), through
    // x + radius, to the bottom.
    half.arc(top, bottom, 180.0);
    let section = format!("{id}_half");
    sketch(b, &section, plane, half)?;
    revolve(b, id, &section, sketch_axis(&section, on_axis), 360.0, Combine::NewBody)?;
    Ok(made("revolve", id))
}
