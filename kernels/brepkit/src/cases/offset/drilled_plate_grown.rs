//! The plate from (−10, −10, 0) to (10, 10, 4), drilled through along the z axis by a hole of
//! radius 3, grown by 1 with its edges rounded.
//!
//! brepkit's rounded offset (`offset_solid_arc_v2`) does not build its arc joints yet, so the
//! drilled plate is offset with sharp joints (the plate grown to (−11, −11, −1)–(11, 11, 5), the
//! hole narrowed to radius 2) and the grown plate's edges are then rounded to radius 1. Each
//! round lies 1 from the two moved faces beside it: the outer edges' rounds on the cylinders of
//! radius 1 about the plate's own edges, closing at its corners on the balls about them, and the
//! hole's rims, concave, filled by the inner quarters of the tubes of radius 1 about the plate's
//! own rims.
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_operations::primitives::{make_box, make_cylinder};
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{cut, edges_where, moved, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let plate = make_box(topo, 20.0, 20.0, 4.0)?;
    let plate = moved(topo, plate, -10.0, -10.0, 0.0)?;
    // the drill runs past both faces
    let drill = make_cylinder(topo, 3.0, 6.0)?;
    let drill = moved(topo, drill, 0.0, 0.0, -1.0)?;
    let drilled = cut(topo, plate, drill)?;
    let grown = offset_solid_v2(topo, drilled, 1.0)?;
    // the edges round the grown plate's two faces, the hole's rims among them, and its four
    // upright corners (not the bore's seam)
    let mut edges = edges_where(topo, grown, |q| near(q.z(), -1.0) || near(q.z(), 5.0))?;
    edges.extend(edges_where(topo, grown, |q| {
        near(q.x().abs(), 11.0) && near(q.y().abs(), 11.0)
    })?);
    Ok(all_blended(fillet_v2(topo, grown, &edges, 1.0)?)?)
}
