//! A plate from (−10, −10, 0) to (10, 10, 2) with a hole of radius 3 bored through it on the z
//! axis (the boring cylinder runs past both faces); both of the hole's circles, on the top and the
//! bottom face, rounded to radius 1 in one fillet, so the rounds meet at the hole's waist.
use brepkit_operations::primitives::{make_box, make_cylinder};
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::rounded;
use crate::solids::{cut, edges_where, moved, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let plate = make_box(topo, 20.0, 20.0, 2.0)?;
    let plate = moved(topo, plate, -10.0, -10.0, 0.0)?;
    let bore = make_cylinder(topo, 3.0, 4.0)?;
    let bore = moved(topo, bore, 0.0, 0.0, -1.0)?;
    let holed = cut(topo, plate, bore)?;
    // the hole's two circles: every point 3 from the z axis, at z = 0 or z = 2
    let rims = edges_where(topo, holed, |q| {
        (q.x().hypot(q.y()) - 3.0).abs() < 1e-7 && (near(q.z(), 0.0) || near(q.z(), 2.0))
    })?;
    Ok(rounded(topo, holed, &rims, 1.0)?)
}
