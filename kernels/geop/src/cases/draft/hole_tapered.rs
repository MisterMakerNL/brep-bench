//! The block from (−15, −15, 0) to (15, 15, 20), bored through along the z axis by a hole of
//! radius 6, its wall drafted 3° about the floor, pulled along +z, so the hole widens as it rises:
//! the block less a frustum of a cone, radius 6 at z = 0. geop drafts only planar faces, so the
//! hole is drafted as it is cut: the circle of radius 6 sketched on the xy plane, extruded 25 up
//! and 5 down, both past the block, with its wall drafted −3° (widening away from the sketch's
//! plane, a cone about its axis), and cut from the block.
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_extrude_revolve::{Extents, ExtrudeArgs, Output};
use geop_ops_model::{Combine, Extent};
use geop_ops_model::sketch::whole_sketch;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([-15.0, -15.0], [15.0, 15.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "block", "base", 20.0, Combine::NewBody)?;
    let mut bore = Drawing::new();
    bore.circle([0.0, 0.0], 6.0);
    sketch(b, "bore", xy(), bore)?;
    b.step(
        "hole",
        ExtrudeArgs {
            sketch: whole_sketch("bore"),
            extent: Extents { side2: Some(Extent::blind(5.0)), ..Extents::blind(25.0) },
            output: Output::Solid(Combine::cut(made("extrude", "block"))),
            draft: -3.0,
        },
    )
}
