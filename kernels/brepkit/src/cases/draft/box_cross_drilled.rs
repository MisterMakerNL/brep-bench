//! The box from the origin to (30, 20, 20) less a hole of radius 4 along x on the line y = 10,
//! z = 10 (the drilling cylinder runs past both ends), then its faces x = 0 and x = 30, where the
//! hole comes out, drafted by 8° with the pull up +z about the neutral plane z = 0, the floor.
use brepkit_math::mat::Mat4;
use brepkit_operations::primitives::{make_box, make_cylinder};
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::{drafted_up, faces_on};
use crate::solids::{cut, near, placed};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 30.0, 20.0, 20.0)?;
    // the drill: a cylinder on z turned onto x, from x = −1 to 31
    let drill = make_cylinder(topo, 4.0, 32.0)?;
    let drill = placed(
        topo,
        drill,
        Mat4::translation(-1.0, 10.0, 10.0) * Mat4::rotation_y(std::f64::consts::FRAC_PI_2),
    )?;
    let drilled = cut(topo, block, drill)?;
    let ends = faces_on(
        topo,
        drilled,
        &[&|q| near(q.x(), 0.0), &|q| near(q.x(), 30.0)],
    )?;
    Ok(drafted_up(topo, drilled, &ends, 0.0, 8.0)?)
}
