//! A square column from z = 0 to 40, its section at height z the square of side 30 − z/2 centred
//! on the z axis, turned right-handed 4.5° for every unit of height: from 30 × 30 to 10 × 10,
//! half a turn in all.
//!
//! Its corners' guides are conical spirals, which geop cannot draw; but the section changes
//! evenly along a straight path, which is geop's sweep twisted and scaled: the square sketched on
//! the xy plane, swept up a line from the origin to (0, 0, 40), turned 180° and scaled to 1/3.
use crate::shared::paths::path2d;
use crate::shared::{Drawing, sketch, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_lib_sweep::Orientation;
use geop_ops_extrude_revolve::{Output, SweepArgs};
use geop_ops_model::Combine;
use geop_ops_model::sketch::whole_sketch;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut square = Drawing::new();
    square.rectangle([-15.0, -15.0], [15.0, 15.0]);
    sketch(b, "square", xy(), square)?;
    let mut axis = Drawing::new();
    let (foot, top) = (axis.point(zx_at(0.0, 0.0)), axis.point(zx_at(0.0, 40.0)));
    axis.line(foot, top);
    sketch(b, "axis", zx(), axis)?;
    b.step(
        "column",
        SweepArgs {
            profile: whole_sketch("square"),
            path: Some(path2d("axis")),
            orientation: Orientation::FollowPath,
            twist: 180.0,
            end_scale: 1.0 / 3.0,
            rails: Vec::new(),
            output: Output::Solid(Combine::NewBody),
        },
    )
}
