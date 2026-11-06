//! The box from the origin to (30, 20, 10), its vertical edges rounded to radius 4 (the rounded
//! rectangle of its floor extruded 10 up), with all eight sides drafted 5° for the pull direction
//! +z about the neutral plane z = 0, the floor: the kernel's draft turns each plane about its
//! bottom edge and makes each round the quarter cone through its quarter circle, tangent to the
//! planes beside it.
use crate::solids::draft::{rounded_rectangle, walls};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{draft_faces, extrude_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let floor = rounded_rectangle(0.0, 0.0, 30.0, 20.0, 4.0, 0.0)?;
    let solid = extrude_profile_brep(&floor, p(0.0, 0.0, 1.0), 10.0)?;
    // Every side: the faces that run along z.
    let sides = walls(&solid, |_| true)?;
    Ok(draft_faces(&solid, &sides, p(0.0, 0.0, 1.0), p(0.0, 0.0, 0.0), 5f64.to_radians())?)
}
