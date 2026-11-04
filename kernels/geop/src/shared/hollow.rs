//! What the shell and offset cases share: the solids they start from, built as a designer
//! builds them (a block extruded from its rectangle, a body turned from its section), geop's
//! shell and face offset written briefly, and the faces, edges and corners they work on picked
//! as a designer picks them: by a point on each, all of a face or edge geop keeps in pieces.
use crate::shared::{Drawing, extrude, made, revolve, sketch, sketch_axis, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::{DatumComponent, FrameAxis};
use geop_core_math::{scalars::Scalar, vector::Vector3};
use geop_core_topology::distance::{Feature, PlacedFace, closest_points};
use geop_ops_edit::OffsetFaceArgs;
use geop_ops_model::datums::{ORIGIN, component_name, datum};
use geop_ops_model::selection::Selected;
use geop_ops_model::sketch::SketchPlane;
use geop_ops_model::{Combine, element, solid};
use geop_ops_reference_geometry::{AddDatumArgs, Construction};
use geop_ops_shell::ShellArgs;

/// The block from `(x0, y0, 0)` to `(x1, y1, height)`: its bottom's rectangle sketched on the
/// xy plane and extruded up, as the step `id`. Its solid's name is returned.
pub fn block(
    b: &mut Bench,
    id: &str,
    from: [f64; 2],
    to: [f64; 2],
    height: f64,
) -> Result<String, Refusal> {
    prism(b, id, &[from, [to[0], from[1]], to, [from[0], to[1]]], height)
}

/// The prism whose section is the polygon through `corners` in the xy plane, from z = 0 up to
/// `height`: the polygon sketched on the xy plane (as `{id}_base`) and extruded, as the step
/// `id`. Its solid's name is returned.
pub fn prism(
    b: &mut Bench,
    id: &str,
    corners: &[[f64; 2]],
    height: f64,
) -> Result<String, Refusal> {
    let mut base = Drawing::new();
    base.polygon(corners);
    let sketch_id = format!("{id}_base");
    sketch(b, &sketch_id, xy(), base)?;
    extrude(b, id, &sketch_id, height, Combine::NewBody)?;
    Ok(made("extrude", id))
}

/// The solid of revolution about the z axis whose half section in the xz plane, on the side
/// x ≥ 0, is the loop `section` draws: `section` is given the drawing and returns the line along
/// the z axis it drew, which the section is turned about a full turn as the step `id`.
pub fn turned(
    b: &mut Bench,
    id: &str,
    section: impl FnOnce(&mut Drawing) -> geop_core_sketch::CurveId,
) -> Result<String, Refusal> {
    let mut d = Drawing::new();
    let on_axis = section(&mut d);
    let sketch_id = format!("{id}_section");
    sketch(b, &sketch_id, zx(), d)?;
    revolve(b, id, &sketch_id, sketch_axis(&sketch_id, on_axis), 360.0, Combine::NewBody)?;
    Ok(made("revolve", id))
}

/// The point `(x, z)` of the xz plane, as the sketches of [`turned`] draw it.
pub fn at(x: f64, z: f64) -> [f64; 2] {
    zx_at(x, z)
}

/// Shells the part's one solid as the step `id`: hollowed to walls `thickness` thick (grown
/// outward from its faces if `outward`), open at the faces through the points `open`. The
/// result is the solid `shell(id)`.
pub fn shell(
    b: &mut Bench,
    id: &str,
    open: &[[f64; 3]],
    thickness: f64,
    outward: bool,
) -> Result<(), Refusal> {
    let body = b.solid()?;
    let faces = whole_faces(b, open)?.into_iter().map(element).collect();
    b.step(
        id,
        ShellArgs {
            solid: solid(body),
            faces,
            thickness: thickness.into(),
            other_faces: Vec::new(),
            other_thickness: 0.2.into(),
            outward,
        },
    )
}

/// Offsets the faces through the points `faces` of the part's one solid `distance` along their
/// outward normals (inward if negative), as the step `id`, the solid rebuilt around them: every
/// edge stays where its two faces, moved, meet.
pub fn offset_faces(
    b: &mut Bench,
    id: &str,
    faces: &[[f64; 3]],
    distance: f64,
) -> Result<(), Refusal> {
    let faces = whole_faces(b, faces)?.into_iter().map(element).collect();
    b.step(id, OffsetFaceArgs { faces, distance })
}

/// The names of every face of the part through any of `points`, each once: a face as a designer
/// picks it, all of it. geop keeps a turned or extruded round face in pieces (a quarter turn
/// each), so a point on the axis of a turned disc picks all of its pieces, and a round side is
/// picked by points round it. A point on no face is an error.
pub fn whole_faces(b: &Bench, points: &[[f64; 3]]) -> Result<Vec<String>, Refusal> {
    let part = b.model()?;
    let model = part.topology();
    let mut found: Vec<String> = Vec::new();
    for &point in points {
        let at = Feature::Point(Vector3::from_array(point.map(Scalar::from_f64)));
        let mut any = false;
        for &id in model.faces.keys() {
            let face = Feature::Face(PlacedFace::new(model, id, None)?);
            if closest_points(&at, &face)?.distance.lower().to_f64() < crate::bench::PICK_TOLERANCE
            {
                any = true;
                let name = part.name(id)?.to_owned();
                if !found.contains(&name) {
                    found.push(name);
                }
            }
        }
        if !any {
            return Err(format!("no face through {point:?}").into());
        }
    }
    Ok(found)
}

/// Points round the z axis at radius `r` and height `z`, an eighth of a turn apart and off the
/// quarter turns where geop's pieces of a round face or edge meet: enough to pick every piece of
/// one, and no edge across it.
pub fn round(r: f64, z: f64) -> Vec<[f64; 3]> {
    (0..8)
        .map(|k| {
            let a = (22.5 + 45.0 * k as f64).to_radians();
            [r * a.cos(), r * a.sin(), z]
        })
        .collect()
}

/// The plane `height` above the xy plane, as the datum step `id` (an offset plane of the
/// origin's xy plane), to sketch on: a sketch on it is drawn in `(x, y)`.
pub fn level(b: &mut Bench, id: &str, height: f64) -> Result<SketchPlane, Refusal> {
    b.step(
        id,
        AddDatumArgs {
            selection: vec![Selected::Datum(datum(component_name(
                ORIGIN,
                DatumComponent::Plane(FrameAxis::Z),
            )))],
            construction: Construction::Offset { distance: height.into() },
        },
    )?;
    Ok(SketchPlane::Datum(datum(id)))
}

/// The names of every edge of the part through any of `points`, each once: an edge as a
/// designer picks it, all of it, a circle geop keeps in pieces picked by points round it.
pub fn whole_edges(b: &Bench, points: &[[f64; 3]]) -> Result<Vec<String>, Refusal> {
    let part = b.model()?;
    let model = part.topology();
    let mut found: Vec<String> = Vec::new();
    for &point in points {
        let at = Feature::Point(Vector3::from_array(point.map(Scalar::from_f64)));
        let mut any = false;
        for (&id, edge) in model.edges.iter() {
            let d = closest_points(&at, &Feature::Curve(edge.curve.clone()))?;
            if d.distance.lower().to_f64() < crate::bench::PICK_TOLERANCE {
                any = true;
                let name = part.name(id)?.to_owned();
                if !found.contains(&name) {
                    found.push(name);
                }
            }
        }
        if !any {
            return Err(format!("no edge through {point:?}").into());
        }
    }
    Ok(found)
}

/// The point of the torus about the z axis whose tube has radius `r` about a middle circle of
/// radius `big`, `around` degrees round the axis from +x and `up` degrees round its tube from its
/// outer equator: a point to pick its face by, off the lines where geop's pieces of a turned
/// circle meet.
pub fn on_torus(big: f64, r: f64, around: f64, up: f64) -> [f64; 3] {
    let (around, up) = (around.to_radians(), up.to_radians());
    let reach = big + r * up.cos();
    [reach * around.cos(), reach * around.sin(), r * up.sin()]
}

/// The name of the part's vertex at `point`, as a designer clicks a corner.
pub fn vertex(b: &Bench, point: [f64; 3]) -> Result<String, Refusal> {
    let part = b.model()?;
    let at = Vector3::from_array(point.map(Scalar::from_f64));
    let mut found = Vec::new();
    for (&id, v) in part.topology().vertices.iter() {
        if v.point.sub(&at).norm().upper().to_f64() < crate::bench::PICK_TOLERANCE {
            found.push(part.name(id)?.to_owned());
        }
    }
    match &found[..] {
        [v] => Ok(v.clone()),
        found => Err(format!("{} vertices at {point:?}: {found:?}", found.len()).into()),
    }
}

/// The names of every edge of the part, as a designer selects them all.
pub fn all_edges(b: &Bench) -> Result<Vec<String>, Refusal> {
    let part = b.model()?;
    let ids: Vec<_> = part.topology().edges.keys().copied().collect();
    ids.into_iter().map(|id| Ok(part.name(id)?.to_owned())).collect()
}
