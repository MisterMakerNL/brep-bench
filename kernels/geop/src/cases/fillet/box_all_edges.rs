//! A cube of side 10 with all twelve edges rounded to radius 1, three rounds meeting at each
//! corner: the cube extruded from a square, and one fillet of its twelve edges, each picked by its
//! midpoint. geop rounds each corner by the ball touching its three faces.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "cube", "base", 10.0, Combine::NewBody)?;
    let mut midpoints = Vec::new();
    for a in [0.0, 10.0] {
        for c in [0.0, 10.0] {
            midpoints.push([5.0, a, c]);
            midpoints.push([a, 5.0, c]);
            midpoints.push([a, c, 5.0]);
        }
    }
    let edges = b.edges(&midpoints)?;
    b.step("round", FilletArgs::constant(edges, 1.0))
}
