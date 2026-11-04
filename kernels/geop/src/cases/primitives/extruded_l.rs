//! An L-shaped plate: the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15),
//! sketched on the xy plane and extruded 8 along +z.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outline = Drawing::new();
    outline.polygon(&[[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0], [0.0, 15.0]]);
    sketch(b, "outline", xy(), outline)?;
    extrude(b, "plate", "outline", 8.0, Combine::NewBody)
}
