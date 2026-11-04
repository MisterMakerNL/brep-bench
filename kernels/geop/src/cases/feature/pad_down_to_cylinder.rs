//! A cylinder of radius 10 along the x axis from x = −20 to x = 20, and a square pad, |x| ≤ 3,
//! |y| ≤ 3 in the plane z = 15, extruded down until it meets the cylinder's side and joined to
//! it. The cylinder is a circle on the yz plane extruded 20 each way; the square is sketched on a
//! datum plane at z = 15 and extruded the other way from its normal, "up to next": until it meets
//! the cylinder it is joined to.
use crate::shared::blends::offset_plane;
use crate::shared::{Drawing, extrude_symmetric, made, sketch, yz};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_extrude_revolve::{Extents, ExtrudeArgs, Output};
use geop_ops_model::sketch::whole_sketch;
use geop_ops_model::{Combine, Extent};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut end = Drawing::new();
    end.circle([0.0, 0.0], 10.0);
    sketch(b, "end", yz(), end)?;
    extrude_symmetric(b, "bar", "end", 40.0, Combine::NewBody)?;
    let above = offset_plane(b, "above", FrameAxis::Z, 15.0)?;
    let mut square = Drawing::new();
    square.rectangle([-3.0, -3.0], [3.0, 3.0]);
    sketch(b, "square", above, square)?;
    b.step(
        "pad",
        ExtrudeArgs {
            sketch: whole_sketch("square"),
            extent: Extents { side1: Extent::UpToNext, reversed: true, ..Extents::blind(1.0) },
            output: Output::Solid(Combine::join(made("extrude", "bar"))),
            draft: 0.0,
        },
    )
}
