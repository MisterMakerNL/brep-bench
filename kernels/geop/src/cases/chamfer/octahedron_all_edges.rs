//! The octahedron |x| + |y| + |z| ≤ 10, all twelve edges chamfered with distance √6 on both
//! faces, and each corner cut flat by the square through the four triangle corners nearest it,
//! in the planes |x| = 6, |y| = 6 and |z| = 6. geop's chamfer leaves a corner to its chamfers,
//! which cross in a point, and has no corner face of its own: so the octahedron (built from the
//! slabs between its opposite faces, see [`octahedron`]) has its twelve edges chamfered in one
//! step, mitred, and then each corner is cut by its plane, as the case composes it: a datum plane
//! offset 6 from the origin's, and a disc on it extruded outward past the corner, cut away.
use crate::shared::blends::{octahedron, offset_plane};
use crate::shared::{Drawing, extrude, sketch};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_fillet::ChamferArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    octahedron(b, "octa", 10.0)?;
    let edges = b.edges(&[
        [5.0, 5.0, 0.0], [-5.0, 5.0, 0.0], [-5.0, -5.0, 0.0], [5.0, -5.0, 0.0],
        [5.0, 0.0, 5.0], [-5.0, 0.0, 5.0], [0.0, 5.0, 5.0], [0.0, -5.0, 5.0],
        [5.0, 0.0, -5.0], [-5.0, 0.0, -5.0], [0.0, 5.0, -5.0], [0.0, -5.0, -5.0],
    ])?;
    b.step("bevel", ChamferArgs::even(edges, 6f64.sqrt()))?;

    for (axis, name) in [(FrameAxis::X, "x"), (FrameAxis::Y, "y"), (FrameAxis::Z, "z")] {
        for (sign, side) in [(1.0, "p"), (-1.0, "n")] {
            let plane = offset_plane(b, &format!("corner_{side}{name}"), axis, 6.0 * sign)?;
            // A disc about the corner's axis, wider than the square, extruded out past the
            // mitre's point, 4 beyond the plane, and cut away.
            let mut disc = Drawing::new();
            disc.circle([0.0, 0.0], 5.0);
            let tip = format!("tip_{side}{name}");
            sketch(b, &tip, plane, disc)?;
            let solid = b.solid()?;
            extrude(b, &format!("flat_{side}{name}"), &tip, 4.0 * sign, Combine::cut(solid))?;
        }
    }
    Ok(())
}
