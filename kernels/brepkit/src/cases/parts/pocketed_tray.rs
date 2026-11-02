//! An open tray on its back, in the case's three steps: the block from (−36, 0, −24) to
//! (36, 16, 24); the pocket from (−33, 0, −21) to (33, 13, 21) cut out of it, the cutting box run
//! past the face y = 0; then four standoffs fused onto the floor's inside face y = 13, each a tube
//! along y from y = 5 to y = 13, a post of radius 5 less its bore of radius 2 (the bore cut from
//! the post alone, past both its ends, so the floor under it stays whole).
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::{block, cylinder_y};
use crate::solids::{cut, fuse};
use crate::Refusal;

const STANDOFFS: [(f64, f64); 4] = [(26.0, 14.0), (-26.0, 14.0), (-26.0, -14.0), (26.0, -14.0)];

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block_ = block(topo, (-36.0, 0.0, -24.0), (36.0, 16.0, 24.0))?;
    let pocket = block(topo, (-33.0, -1.0, -21.0), (33.0, 13.0, 21.0))?;
    let mut tray = cut(topo, block_, pocket)?;
    for (x, z) in STANDOFFS {
        let post = cylinder_y(topo, 5.0, (x, z), 5.0, 13.0)?;
        let bore = cylinder_y(topo, 2.0, (x, z), 4.0, 14.0)?;
        let tube = cut(topo, post, bore)?;
        tray = fuse(topo, tray, tube)?;
    }
    Ok(tray)
}
