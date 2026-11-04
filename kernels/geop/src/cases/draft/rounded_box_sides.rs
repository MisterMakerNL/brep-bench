//! The box from the origin to (30, 20, 10) with its four vertical edges rounded to radius 4, all
//! eight sides drafted 5° about the plane of its floor, pulled along +z: the planes turned about
//! their bottom edges and the quarter cylinders become quarter cones about their own axes. geop
//! drafts only planar faces, and none that meets a neighbour tangentially, so the box is drafted
//! as it is made: the rounded rectangle sketched on the xy plane and extruded 10 up with its walls
//! drafted 5°, a line's wall a plane at the angle and an arc's a cone about the arc's axis.
use crate::shared::blends::rounded_rectangle;
use crate::shared::{Drawing, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_extrude_revolve::{Extents, ExtrudeArgs, Output};
use geop_ops_model::Combine;
use geop_ops_model::sketch::whole_sketch;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outline = Drawing::new();
    rounded_rectangle(&mut outline, [0.0, 0.0], [30.0, 20.0], 4.0);
    sketch(b, "outline", xy(), outline)?;
    b.step(
        "box",
        ExtrudeArgs {
            sketch: whole_sketch("outline"),
            extent: Extents::blind(10.0),
            output: Output::Solid(Combine::NewBody),
            draft: 5.0,
        },
    )
}
