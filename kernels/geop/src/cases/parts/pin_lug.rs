//! A flat lug 80 long in x, 40 tall in z and 10 thick in y, with a pin hole of radius 8 through
//! its thickness on the line x = 60, z = 20: the plate's outline sketched on the xz plane and
//! extruded 5 each way along y, and the hole's circle extruded 10 each way, past both faces, and
//! cut from it.
use crate::shared::{Drawing, extrude_symmetric, made, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outline = Drawing::new();
    outline.rectangle(zx_at(0.0, 0.0), zx_at(80.0, 40.0));
    sketch(b, "outline", zx(), outline)?;
    extrude_symmetric(b, "plate", "outline", 10.0, Combine::NewBody)?;

    let mut pin = Drawing::new();
    pin.circle(zx_at(60.0, 20.0), 8.0);
    sketch(b, "pin", zx(), pin)?;
    extrude_symmetric(b, "hole", "pin", 20.0, Combine::cut(made("extrude", "plate")))
}
