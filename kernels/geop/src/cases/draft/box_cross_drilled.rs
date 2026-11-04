//! The box from the origin to (30, 20, 20), drilled through along x by a hole of radius 4 on the
//! line y = 10, z = 10; then its two faces x = 0 and x = 30, each with the hole's circle in it,
//! drafted 8° about the plane of the floor, pulled along +z. The box is extruded, the hole cut by
//! a circle on the yz plane extruded past both ends, and the two faces drafted about the xy plane:
//! the hole's wall stays as it is, trimmed by the drafted faces.
use crate::shared::blends::draft;
use crate::shared::{Drawing, extrude, extrude_symmetric, made, sketch, xy, yz};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [30.0, 20.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "box", "base", 20.0, Combine::NewBody)?;
    let mut bore = Drawing::new();
    bore.circle([10.0, 10.0], 4.0);
    sketch(b, "bore", yz(), bore)?;
    extrude_symmetric(b, "hole", "bore", 80.0, Combine::cut(made("extrude", "box")))?;
    let ends = b.faces(&[[0.0, 2.0, 2.0], [30.0, 2.0, 2.0]])?;
    b.step("taper", draft(ends, xy(), 8.0))
}
