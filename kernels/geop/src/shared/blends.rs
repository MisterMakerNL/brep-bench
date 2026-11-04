//! What the blend, draft and edit cases share besides [`super`]: a vertex picked by its point, as
//! an edge or a face is, a rounded rectangle drawn in a sketch, and a few steps written briefly.
use super::Drawing;
use crate::{Bench, Refusal};
use geop_core_program::element::ElementReference;
use geop_ops_model::model;

/// How near a point must be to a vertex to pick it.
const PICK_TOLERANCE: f64 = 1e-6;

/// The name of the one vertex of the part at `point`, as a designer picks it.
pub fn vertex(b: &Bench, point: [f64; 3]) -> Result<String, Refusal> {
    let part = b.model()?;
    let mut found = Vec::new();
    for (&id, v) in part.topology().vertices.iter() {
        let d = (0..3).map(|i| (v.point[i].to_f64() - point[i]).powi(2)).sum::<f64>().sqrt();
        if d < PICK_TOLERANCE {
            found.push(part.name(id)?.to_owned());
        }
    }
    match &found[..] {
        [v] => Ok(v.clone()),
        found => Err(format!("{} vertices at {point:?}: {found:?}", found.len()).into()),
    }
}

/// A reference to the element named `name` of the part, of whatever kind `K` it is.
pub fn element<K>(name: String) -> ElementReference<geop_ops_model::NamedModel, K> {
    ElementReference::new(model(), name)
}

/// The rectangle from corner `a` to the opposite corner `b`, its sides parallel to the axes and
/// its four corners rounded to `radius`, drawn counter-clockwise: each side, then the quarter
/// circle turning into the next.
pub fn rounded_rectangle(d: &mut Drawing, a: [f64; 2], b: [f64; 2], radius: f64) {
    let r = radius;
    // Each side's two ends, counter-clockwise from the bottom side.
    let sides = [
        [[a[0] + r, a[1]], [b[0] - r, a[1]]],
        [[b[0], a[1] + r], [b[0], b[1] - r]],
        [[b[0] - r, b[1]], [a[0] + r, b[1]]],
        [[a[0], b[1] - r], [a[0], a[1] + r]],
    ];
    let ends: Vec<_> = sides.iter().map(|[p, q]| (d.point(*p), d.point(*q))).collect();
    for i in 0..4 {
        d.line(ends[i].0, ends[i].1);
        d.arc(ends[i].1, ends[(i + 1) % 4].0, 90.0);
    }
}

/// Adds the datum plane `id`, the origin's plane normal to `normal` offset `distance` along it,
/// and returns it as a plane to sketch on or to draft about.
pub fn offset_plane(
    b: &mut Bench,
    id: &str,
    normal: geop_core_math::primitives::FrameAxis,
    distance: f64,
) -> Result<geop_ops_model::sketch::SketchPlane, Refusal> {
    use geop_core_math::primitives::DatumComponent;
    use geop_ops_model::datums::{ORIGIN, component_name, datum};
    use geop_ops_reference_geometry::{AddDatumArgs, Construction};
    let origin = component_name(ORIGIN, DatumComponent::Plane(normal));
    b.step(
        id,
        AddDatumArgs {
            selection: vec![geop_ops_model::selection::Selected::Datum(datum(origin))],
            construction: Construction::Offset { distance: distance.into() },
        },
    )?;
    Ok(geop_ops_model::sketch::SketchPlane::Datum(datum(id)))
}

/// Adds the datum point `id`, the origin moved by `at` along its axes.
pub fn datum_point(b: &mut Bench, id: &str, at: [f64; 3]) -> Result<(), Refusal> {
    use geop_ops_model::datums::{ORIGIN, datum};
    use geop_ops_reference_geometry::{AddDatumArgs, Construction};
    b.step(
        id,
        AddDatumArgs {
            selection: vec![geop_ops_model::selection::Selected::Datum(datum(ORIGIN))],
            construction: Construction::Point { x: at[0].into(), y: at[1].into(), z: at[2].into() },
        },
    )
}

/// The octahedron |x| + |y| + |z| ≤ `r`, as geop builds one, having no polyhedra of its own: its
/// six corners as datum points on the axes, and for each pair of opposite faces the slab between
/// them — a disc on the datum plane through the corners of one face, wide enough to cover the
/// whole octahedron, extruded back to the opposite face — the four slabs intersected. Its steps
/// are named after `id`; the octahedron is the solid it returns the name of.
pub fn octahedron(b: &mut Bench, id: &str, r: f64) -> Result<String, Refusal> {
    use super::{extrude, made, sketch};
    use geop_ops_model::Combine;
    use geop_ops_model::datums::datum;
    use geop_ops_model::selection::Selected;
    use geop_ops_reference_geometry::{AddDatumArgs, Construction};
    let corner = |axis: usize, sign: f64| {
        let name = format!("{id}_{}{}", if sign > 0.0 { "p" } else { "n" }, ["x", "y", "z"][axis]);
        let mut at = [0.0; 3];
        at[axis] = sign * r;
        (name, at)
    };
    for axis in 0..3 {
        for sign in [1.0, -1.0] {
            let (name, at) = corner(axis, sign);
            datum_point(b, &name, at)?;
        }
    }
    // The faces s · (x, y, z) = r with s's signs multiplying to +1: through the corners at
    // s_x r on x, s_y r on y and s_z r on z, in that order, the plane's normal (b − a) × (c − a)
    // is along s, out of the octahedron. Each slab runs back from it across the octahedron, to the
    // opposite face, 2r/√3 away.
    let mut solid = String::new();
    for (k, s) in [[1.0, 1.0, 1.0], [1.0, -1.0, -1.0], [-1.0, 1.0, -1.0], [-1.0, -1.0, 1.0]]
        .into_iter()
        .enumerate()
    {
        let plane = format!("{id}_face{k}");
        b.step(
            &plane,
            AddDatumArgs {
                selection: (0..3).map(|i| Selected::Datum(datum(corner(i, s[i]).0))).collect(),
                construction: Construction::ThreePoints {},
            },
        )?;
        let mut disc = super::Drawing::new();
        disc.circle([0.0, 0.0], 3.0 * r);
        let section = format!("{id}_section{k}");
        sketch(b, &section, geop_ops_model::sketch::SketchPlane::Datum(datum(plane.as_str())), disc)?;
        let slab = format!("{id}_slab{k}");
        let combine = if k == 0 { Combine::NewBody } else { Combine::intersect(solid.clone()) };
        extrude(b, &slab, &section, -2.0 * r / 3f64.sqrt(), combine)?;
        solid = made("extrude", &slab);
    }
    Ok(solid)
}

/// The draft of the faces `faces` by `degrees` about the plane `neutral`, pulled along its
/// normal: each face leans that far, inward as it goes along the pull (outward if negative).
pub fn draft(
    faces: Vec<String>,
    neutral: geop_ops_model::sketch::SketchPlane,
    degrees: f64,
) -> geop_ops_plastic::DraftArgs {
    geop_ops_plastic::DraftArgs {
        faces: faces.into_iter().map(element).collect(),
        neutral: Some(neutral),
        angle: degrees,
        reversed: false,
    }
}
