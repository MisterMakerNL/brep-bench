//! The solid a box fills as it slides without turning along (8, 5, 3): the box from the origin to
//! (10, 6, 4), and everything it passes through on its way to (8, 5, 3)–(18, 11, 7).
//!
//! geop sweeps a profile, not a solid, so the solid is built as what it is: the box where it
//! starts, joined by what its three faces facing along the slide (x = 10, y = 6, z = 4) sweep —
//! each face swept along the slide without turning, an oblique prism. Every point the box passes
//! leaves it, going along the slide, through one of those faces, so it lies in the box or in what
//! a face sweeps.
use crate::shared::paths::{at, offset_plane, path3d, sketch3d};
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_lib_sweep::Orientation;
use geop_ops_extrude_revolve::{Output, SweepArgs};
use geop_ops_model::Combine;
use geop_ops_model::sketch::whole_sketch;
use geop_ops_model::sketch3d::Sketch3d;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 6.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "start", "base", 4.0, Combine::NewBody)?;

    let slide = [8.0, 5.0, 3.0];
    // Each face facing along the slide: its plane, normal to an axis at a distance, the face
    // drawn on it (the yz plane drawn in (y, z), the zx in (x, −z), the xy in (x, y)), and its
    // centre, where its path starts.
    let faces = [
        ("face_x", FrameAxis::X, 10.0, [[0.0, 0.0], [6.0, 4.0]], [10.0, 3.0, 2.0]),
        ("face_y", FrameAxis::Y, 6.0, [[0.0, 0.0], [10.0, -4.0]], [5.0, 6.0, 2.0]),
        ("face_z", FrameAxis::Z, 4.0, [[0.0, 0.0], [10.0, 6.0]], [5.0, 3.0, 4.0]),
    ];
    for (id, normal, distance, [a, c], centre) in faces {
        let plane = offset_plane(b, &format!("{id}_plane"), normal, distance)?;
        let mut face = Drawing::new();
        face.rectangle(a, c);
        sketch(b, id, plane, face)?;
        let mut line = Sketch3d::new();
        let from = line.add_point(at(centre));
        let to = line.add_point(at([0, 1, 2].map(|k| centre[k] + slide[k])));
        line.add_line(from, to);
        let path = format!("{id}_slide");
        sketch3d(b, &path, line)?;
        let body = b.solid()?;
        b.step(
            &format!("{id}_swept"),
            SweepArgs {
                profile: whole_sketch(id),
                path: Some(path3d(&path)),
                orientation: Orientation::FixedNormal,
                twist: 0.0,
                end_scale: 1.0,
                rails: Vec::new(),
                output: Output::Solid(Combine::join(body)),
            },
        )?;
    }
    Ok(())
}
