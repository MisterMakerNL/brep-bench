//! A ball of radius 5 centred on the origin, with a hole of radius 3 bored right through it along
//! the z axis: the ball turned from a half disc, and a circle of radius 3 on the xy plane
//! extruded 10 each way, past the ball at both ends, cut from it.
use crate::shared::{Drawing, extrude_symmetric, made, revolve, sketch, sketch_axis, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut half = Drawing::new();
    let bottom = half.point(zx_at(0.0, -5.0));
    let top = half.point(zx_at(0.0, 5.0));
    let on_axis = half.line(bottom, top);
    half.arc(top, bottom, 180.0);
    sketch(b, "half", zx(), half)?;
    revolve(b, "ball", "half", sketch_axis("half", on_axis), 360.0, Combine::NewBody)?;

    let mut bore = Drawing::new();
    bore.circle([0.0, 0.0], 3.0);
    sketch(b, "bore", xy(), bore)?;
    extrude_symmetric(b, "hole", "bore", 20.0, Combine::cut(made("revolve", "ball")))
}
