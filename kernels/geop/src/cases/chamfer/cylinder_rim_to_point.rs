//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the rim of its top face chamfered
//! with distance 5 on both faces, the cylinder's own radius, so the top shrinks to a point and the
//! chamfer is a cone with its apex at (0, 0, 10): a circle extruded, and the rim chamfered.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::ChamferArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.circle([0.0, 0.0], 5.0);
    sketch(b, "base", xy(), base)?;
    extrude(b, "drum", "base", 10.0, Combine::NewBody)?;
    let rim = b.edge([3.0, 4.0, 10.0])?;
    b.step("cone", ChamferArgs::even(vec![rim], 5.0))
}
