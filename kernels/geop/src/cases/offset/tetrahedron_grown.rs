//! The regular tetrahedron with corners (3, 3, 3), (3, −3, −3), (−3, 3, −3) and (−3, −3, 3),
//! grown by 1: every point within 1 of it. Each face moves 1 outward, each edge becomes a piece
//! of a cylinder of radius 1 about it, each corner a piece of a ball of radius 1 about it.
//!
//! The tetrahedron is the cube from (−3, −3, −3) to (3, 3, 3) with its other four corners cut
//! off, each by the plane through the three tetrahedron corners round it: a datum plane through
//! those three corners of the cube, picked, and a rectangle sketched on it extruded out through
//! the cube's corner. geop has no offset of a solid that rounds where moved faces part: its face
//! offset moves the faces and meets them sharp. So the four faces are offset 1 out, and all six
//! edges, selected together, then rounded to radius 1: each round is tangent to two moved faces, so it lies about the
//! tetrahedron's own edge, and where three meet, the corner is the ball about its own corner.
use crate::shared::hollow::{all_edges, level, offset_faces, vertex};
use crate::shared::{Drawing, extrude, made, sketch};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::datums::datum;
use geop_ops_model::selection::Selected;
use geop_ops_model::sketch::SketchPlane;
use geop_ops_model::{Combine, element};
use geop_ops_reference_geometry::{AddDatumArgs, Construction};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let floor = level(b, "floor", -3.0)?;
    let mut base = Drawing::new();
    base.rectangle([-3.0, -3.0], [3.0, 3.0]);
    sketch(b, "base", floor, base)?;
    extrude(b, "cube", "base", 6.0, Combine::NewBody)?;
    let mut body = made("extrude", "cube");

    // Each face's corners in the order that turns the plane's normal, (b − a) × (c − a), away
    // from the tetrahedron, towards the corner of the cube cut off.
    let faces: [[[f64; 3]; 3]; 4] = [
        [[3.0, -3.0, -3.0], [-3.0, -3.0, 3.0], [-3.0, 3.0, -3.0]],
        [[3.0, 3.0, 3.0], [-3.0, 3.0, -3.0], [-3.0, -3.0, 3.0]],
        [[3.0, 3.0, 3.0], [-3.0, -3.0, 3.0], [3.0, -3.0, -3.0]],
        [[3.0, 3.0, 3.0], [3.0, -3.0, -3.0], [-3.0, 3.0, -3.0]],
    ];
    for (i, corners) in faces.iter().enumerate() {
        let plane = format!("face{i}");
        let selection = (corners.iter())
            .map(|&c| Ok(Selected::Vertex(element(vertex(b, c)?))))
            .collect::<Result<Vec<_>, Refusal>>()?;
        b.step(&plane, AddDatumArgs { selection, construction: Construction::ThreePoints {} })?;
        let mut beyond = Drawing::new();
        beyond.rectangle([-20.0, -20.0], [20.0, 20.0]);
        let sketch_id = format!("beyond{i}");
        sketch(b, &sketch_id, SketchPlane::Datum(datum(plane.as_str())), beyond)?;
        let cut = format!("corner{i}");
        extrude(b, &cut, &sketch_id, 20.0, Combine::cut(body))?;
        body = made("extrude", &cut);
    }

    // A point inside each face: the middle of its corners.
    let middles: Vec<[f64; 3]> =
        (faces.iter()).map(|f| [0, 1, 2].map(|k| (f[0][k] + f[1][k] + f[2][k]) / 3.0)).collect();
    offset_faces(b, "grow", &middles, 1.0)?;
    let edges = all_edges(b)?;
    b.step("rounds", FilletArgs::constant(edges, 1.0))
}
