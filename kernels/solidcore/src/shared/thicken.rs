//! What the solidcore implementations of the thicken cases share: the sheets more than one case
//! thickens.
//!
//! solidcore-ops's [`thicken`](solidcore_ops::thicken) is the kernel's shell of a sheet
//! (`SmApiShellBrepFull`) with its convex edges extended: where the moved faces part at a crease
//! they are extended until they meet, a sharp corner, and where they cross they are trimmed where
//! they meet. Ops has no switch for the kernel's other reading, a round about the crease; and the
//! kernel's shell with that reading (`bDoExtendedOffset` false) leaves a sheet's round open at
//! its ends, where a lamina vertex's offset is an arc and no side wall is made for it (the angle
//! bracket's two quarter discs are missing, the solid has no inside; the open tray is refused,
//! `SM_ERR_INVALID_INPUT`), with every upstream fix on or off. The rounded cases thicken sharp and
//! round the corners the thickening made, at the thickness, about the creases.
use crate::bench::{arc, line};
use crate::Refusal;
use solidcore::curve::curve::Curve as KCurve;
use solidcore::curve::line::Line;
use solidcore::geom::tol::USE_DEFAULT;
use solidcore::kernel::Kernel;
use solidcore::sweep::segments::build_faces;
use solidcore::topology::brep::p3::new_brep;
use solidcore::topology::brep::p5::{stitch_and_orient, StitchAndOrientOptions};
use solidcore::topology::brep::p8::create_planar_face_with_3d_curves;
use solidcore::Point3;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;
use std::f64::consts::FRAC_PI_2;

/// The sheet of the angle-bracket cases: the floor from (0, 0, 0) to (10, 8, 0) in the xy plane
/// and the wall from (0, 0, 0) to (0, 8, 10) in the yz plane, sharing the crease along y: the
/// profile from the floor's far edge through the crease to the wall's top, swept along +y
/// uncapped. Its normals face out of the crease, to -z and -x.
pub fn bracket() -> solidcore_ops::Result<Body> {
    let profile =
        [line([10.0, 0.0, 0.0], [0.0, 0.0, 0.0]), line([0.0, 0.0, 0.0], [0.0, 0.0, 10.0])];
    extrude(&profile, [0.0, 1.0, 0.0], 8.0, 0.0, DraftCorner::default(), false)
}

/// The bent sheet of the bent-flange cases: the profile in the xz plane along the line from
/// (-10, 0, 0) to the origin, round the quarter circle of radius 2 about (0, 0, 2) to (2, 0, 2),
/// and up the line to (2, 0, 12), swept along +y from y = 0 to 8 uncapped: a floor, a bend and a
/// wall, meeting tangentially. Its normals face away from the bend's axis.
pub fn flange() -> solidcore_ops::Result<Body> {
    let profile = [
        line([-10.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
        // from -z to +x about (0, 0, 2): counter-clockwise about -y
        arc([0.0, 0.0, 2.0], [0.0, -1.0, 0.0], [0.0, 0.0, -1.0], 2.0, 0.0, FRAC_PI_2),
        line([2.0, 0.0, 2.0], [2.0, 0.0, 12.0]),
    ];
    extrude(&profile, [0.0, 1.0, 0.0], 8.0, 0.0, DraftCorner::default(), false)
}

/// The open tray of the open-box cases: the floor (z = 0) and four walls of the cube from the
/// origin to (10, 10, 10), with no top, each a planar face bounded by its square, sewn into one
/// sheet along their shared edges and oriented alike, their normals out of the cube.
///
/// Ops makes planar faces on one plane only ([`solidcore_ops::sweep::planar_faces`],
/// `SmApiCreatePlanarFaces`, fails for loops on five planes), and has no call that puts faces
/// made apart into one body; so this makes them as ops's own pyramid does
/// ([`solidcore_ops::sweep::loft_to_point`]): each face by the kernel's
/// `CreatePlanarFaceWith3DCurves` in one new B-rep, then `StitchAndOrient`.
pub fn open_box() -> Result<Body, Refusal> {
    let p = |x: f64, y: f64, z: f64| Point3::new(x, y, z);
    let a = 10.0;
    let squares = [
        [p(0.0, 0.0, 0.0), p(0.0, a, 0.0), p(a, a, 0.0), p(a, 0.0, 0.0)],
        [p(0.0, 0.0, 0.0), p(a, 0.0, 0.0), p(a, 0.0, a), p(0.0, 0.0, a)],
        [p(a, 0.0, 0.0), p(a, a, 0.0), p(a, a, a), p(a, 0.0, a)],
        [p(a, a, 0.0), p(0.0, a, 0.0), p(0.0, a, a), p(a, a, a)],
        [p(0.0, a, 0.0), p(0.0, 0.0, 0.0), p(0.0, 0.0, a), p(0.0, a, a)],
    ];
    let mut k = crate::shared::swept::session_kernel();
    let mut b = new_brep(&mut k, USE_DEFAULT, USE_DEFAULT)?;
    let region = b.infinite_region();
    build_faces(&mut b, |b, fenv| -> solidcore::Result<()> {
        for square in &squares {
            let mut sides = Vec::new();
            for i in 0..4 {
                sides.push(KCurve::from(Line::create_line_segment(
                    3,
                    square[i],
                    square[(i + 1) % 4],
                    None,
                )?));
            }
            create_planar_face_with_3d_curves(&mut k, b, region, &sides, 1e-6, fenv)?;
        }
        Ok(())
    })?;
    stitch_and_orient(&mut k, &mut b, &StitchAndOrientOptions::default())?;
    b.refresh_cached_topology();
    let (record, _) = solidcore_convert::brep::to_record(&mut k, &mut b)
        .map_err(|e| format!("the tray's record was not made: {e:?}"))?;
    Ok(Body::from_record(&record, true)?.0)
}
