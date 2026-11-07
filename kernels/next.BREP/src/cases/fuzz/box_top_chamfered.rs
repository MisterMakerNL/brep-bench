//! A box from the origin to (a, b, c), the four edges around its top face chamfered unequally, d1
//! across the top and d2 down the side, each edge its own two-distance chamfer; turned by ψ about
//! the axis (θ, φ).
use crate::solids::fillet::{faces_of_edge_through, lies_in_plane_z};
use crate::solids::fuzz::turned_by;
use crate::solids::p;
use crate::{Params, Refusal};
use brep_kernel::{chamfer_edges_asymmetric, make_box_brep, BrepSolid};

pub fn build(p: &Params) -> Result<BrepSolid, Refusal> {
    let (a, b, c) = (p.num("a")?, p.num("b")?, p.num("c")?);
    let (across_top, down_side) = (p.num("d1")?, p.num("d2")?);
    let mut solid = make_box_brep(self::p(0.0, 0.0, 0.0), a, b, c)?;
    for middle in [self::p(a / 2.0, 0.0, c), self::p(a, b / 2.0, c), self::p(a / 2.0, b, c),
                   self::p(0.0, b / 2.0, c)] {
        // The kernel's two-distance chamfer sets back its first distance on the edge's first
        // face, in the order of the solid's shells: across the top where that is the top.
        let faces = faces_of_edge_through(&solid, middle);
        let top_first = match faces.first() {
            Some(face) => lies_in_plane_z(face, c),
            None => return Err(Refusal::from(format!("no edge through {middle:?}"))),
        };
        let (d1, d2) = if top_first { (across_top, down_side) } else { (down_side, across_top) };
        solid = chamfer_edges_asymmetric(&solid, &[middle], None, d1, d2, None)?;
    }
    Ok(turned_by(&solid, p.num("theta")?, p.num("phi")?, p.num("psi")?)?)
}
