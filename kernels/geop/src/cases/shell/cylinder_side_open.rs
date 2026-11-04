//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, shelled inward to a wall 1 thick,
//! open at its curved side: only the ends are walls, two separate discs 1 thick. The cylinder is
//! a circle extruded, shelled by geop's shell, the side picked by points round it.
use crate::shared::hollow::{round, shell};
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.circle([0.0, 0.0], 5.0);
    sketch(b, "base", xy(), base)?;
    extrude(b, "can", "base", 10.0, Combine::NewBody)?;
    shell(b, "discs", &round(5.0, 5.0), 1.0, false)
}
