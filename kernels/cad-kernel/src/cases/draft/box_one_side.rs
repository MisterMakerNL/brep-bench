//! The box from the origin to (20, 12, 10), its side x = 20 drafted 10° inward about z = 0.
use crate::sketch::{block, drafted_face};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // cad-kernel drafts a profile as it extrudes it, not a solid's faces: the box with one side
    // drafted is the box kept inside that side's plane turned about its bottom edge
    let solid = block([0., 0., 0.], [20., 12., 10.])?;
    drafted_face(&solid, [20., 0., 0.], [1., 0., 0.], 10f64.to_radians())
}
