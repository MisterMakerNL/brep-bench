//! A ball of radius 5 centred on the origin, bored right through along the z axis by a hole of
//! radius 3, its poles put on the rims of the ring, at (3, 0, 4) and (−3, 0, −4). geop has no
//! ball of its own: a half disc in the xz plane, bounded by the diameter from (−3, 0, −4) to
//! (3, 0, 4), is turned about that diameter. The hole is a circle of radius 3 on the xy plane
//! extruded 10 each way, past the ball at both ends, and cut.
use crate::shared::{Drawing, extrude_symmetric, made, revolve, sketch, sketch_axis, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut half = Drawing::new();
    let low = half.point(zx_at(-3.0, -4.0));
    let high = half.point(zx_at(3.0, 4.0));
    let on_axis = half.line(low, high);
    half.arc(high, low, 180.0);
    sketch(b, "half", zx(), half)?;
    revolve(b, "ball", "half", sketch_axis("half", on_axis), 360.0, Combine::NewBody)?;

    let mut bore = Drawing::new();
    bore.circle([0.0, 0.0], 3.0);
    sketch(b, "bore", xy(), bore)?;
    extrude_symmetric(b, "hole", "bore", 20.0, Combine::cut(made("revolve", "ball")))
}
