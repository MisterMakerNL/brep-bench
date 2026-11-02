//! A cube of side 10 between the origin and (10, 10, 10), all twelve edges chamfered with the
//! same distance 2 on both faces, the three chamfers at each corner closed by a flat triangle:
//! the cube cut by the twelve chamfer planes (x + y ≤ 18 and the like) and the eight corner
//! planes (x + y + z ≤ 26 and the like). All twelve in one chamfer, as OCCT's MakeChamfer does.
use crate::bench::{edges, p};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let all = edges(m, cube);
    Ok(blend::chamfer(m, cube, &all, 2.0)?)
}
