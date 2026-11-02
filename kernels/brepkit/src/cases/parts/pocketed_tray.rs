//! An open tray on its back: the block from (−36, 0, −24) to (36, 16, 24), with a pocket taken
//! out of it that leaves walls and a floor 3 thick and opens through the face y = 0, and four
//! standoffs on the floor's inside face y = 13, tubes of radius 5 bored blind to radius 2.
//!
//! The pocket is cut round the standoffs, as islands: the cutting box, from (−33, 0, −21) to
//! (33, 13, 21) and run past the face y = 0, less the four posts of radius 5 from y = 5 to the
//! floor. Then each post's bore of radius 2 is drilled from past its free end down to the floor.
//! This is the solid the case's three steps make (the pocket, then the tubes joined to the
//! floor); in that order, brepkit's fuse of a tube standing on the pocket's floor falls back to a
//! mesh Boolean (and a post sunk into the floor fuses, but its bore then cuts wrong).
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::{block, cylinder_y};
use crate::solids::cut;
use crate::Refusal;

const STANDOFFS: [(f64, f64); 4] = [(26.0, 14.0), (-26.0, 14.0), (-26.0, -14.0), (26.0, -14.0)];

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let tray = block(topo, (-36.0, 0.0, -24.0), (36.0, 16.0, 24.0))?;
    let mut pocket = block(topo, (-33.0, -1.0, -21.0), (33.0, 13.0, 21.0))?;
    for (x, z) in STANDOFFS {
        let post = cylinder_y(topo, 5.0, (x, z), 5.0, 13.0)?;
        pocket = cut(topo, pocket, post)?;
    }
    let mut tray = cut(topo, tray, pocket)?;
    for (x, z) in STANDOFFS {
        let bore = cylinder_y(topo, 2.0, (x, z), 4.0, 13.0)?;
        tray = cut(topo, tray, bore)?;
    }
    Ok(tray)
}
