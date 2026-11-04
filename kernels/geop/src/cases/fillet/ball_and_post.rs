//! A ball of radius 10 centred on the origin and a post, the cylinder of radius 4 on the z axis
//! from z = 0 to z = 15, joined; the circle where the post leaves the ball, radius 4 at z = √84,
//! rounded to radius 2, adding material. The ball is turned from a half disc (geop has no ball of
//! its own), the post extruded from a circle and joined to it, and the edge filleted.
use crate::shared::{Drawing, extrude, made, revolve, sketch, sketch_axis, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut half = Drawing::new();
    let bottom = half.point(zx_at(0.0, -10.0));
    let top = half.point(zx_at(0.0, 10.0));
    let on_axis = half.line(bottom, top);
    half.arc(top, bottom, 180.0);
    sketch(b, "half", zx(), half)?;
    revolve(b, "ball", "half", sketch_axis("half", on_axis), 360.0, Combine::NewBody)?;

    let mut foot = Drawing::new();
    foot.circle([0.0, 0.0], 4.0);
    sketch(b, "foot", xy(), foot)?;
    extrude(b, "post", "foot", 15.0, Combine::join(made("revolve", "ball")))?;

    let neck = b.edge([2.4, 3.2, 84f64.sqrt()])?;
    b.step("round", FilletArgs::constant(vec![neck], 2.0))
}
