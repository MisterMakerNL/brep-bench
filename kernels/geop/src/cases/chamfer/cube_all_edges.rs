//! A cube of side 10 between the origin and (10, 10, 10), all twelve edges chamfered with
//! distance 2 on both faces, and each corner cut flat by the triangle through the three corners
//! of the chamfered faces nearest it. geop's chamfer leaves a corner to its chamfers, which cross
//! in a point, and has no corner face of its own: so the cube is extruded, its twelve edges
//! chamfered in one step (mitred), and then each corner cut by its plane, as the case composes
//! it: a datum plane through the three square corners nearest it, and a disc on that plane
//! extruded outward, cut from the solid.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::shared::blends::{element, vertex};
use crate::{Bench, Refusal};
use geop_ops_fillet::ChamferArgs;
use geop_ops_model::datums::datum;
use geop_ops_model::selection::Selected;
use geop_ops_model::sketch::SketchPlane;
use geop_ops_model::Combine;
use geop_ops_reference_geometry::{AddDatumArgs, Construction};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "cube", "base", 10.0, Combine::NewBody)?;
    let mut midpoints = Vec::new();
    for a in [0.0, 10.0] {
        for c in [0.0, 10.0] {
            midpoints.extend([[5.0, a, c], [a, 5.0, c], [a, c, 5.0]]);
        }
    }
    let edges = b.edges(&midpoints)?;
    b.step("bevel", ChamferArgs::even(edges, 2.0))?;

    let mut k = 0;
    for x in [0.0, 10.0] {
        for y in [0.0, 10.0] {
            for z in [0.0, 10.0] {
                let corner = [x, y, z];
                // The square corners nearest it: 2 in from it along two of its three edges.
                let inward = corner.map(|c| if c == 0.0 { 2.0 } else { -2.0 });
                let near = |skip: usize| {
                    let mut p = corner;
                    (0..3).filter(|&i| i != skip).for_each(|i| p[i] += inward[i]);
                    p
                };
                // Taken round the corner so that the plane's normal, (q − p) × (r − p), points
                // out of the cube: counter-clockwise seen from outside.
                let odd = inward.iter().filter(|&&d| d < 0.0).count() % 2 == 1;
                let (p, q, r) = if odd { (near(2), near(0), near(1)) } else { (near(2), near(1), near(0)) };
                let plane = format!("corner{k}");
                b.step(
                    &plane,
                    AddDatumArgs {
                        selection: [p, q, r]
                            .iter()
                            .map(|&v| Ok(Selected::Vertex(element(vertex(b, v)?))))
                            .collect::<Result<_, Refusal>>()?,
                        construction: Construction::ThreePoints {},
                    },
                )?;
                // A disc about the plane's origin, the first of the three corners, wide enough to
                // cover the triangle, extruded out past the mitre's point and cut away.
                let mut disc = Drawing::new();
                disc.circle([0.0, 0.0], 3.0);
                let tip = format!("tip{k}");
                sketch(b, &tip, SketchPlane::Datum(datum(plane.as_str())), disc)?;
                let solid = b.solid()?;
                extrude(b, &format!("flat{k}"), &tip, 2.0, Combine::cut(solid))?;
                k += 1;
            }
        }
    }
    Ok(())
}
