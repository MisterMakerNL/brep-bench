//! The box from the origin to (30, 20, 10), its vertical edges rounded to radius 4, with all eight
//! sides drafted 5° for the pull direction +z about the neutral plane z = 0, the floor: the kernel's
//! drafted extrusion of the floor's rounded rectangle, 10 up, leaning 5° inward as it rises, its
//! planes turned about their bottom edges and its rounds quarter cones through their quarter
//! circles, tangent to the planes beside them.
use crate::solids::draft::rounded_rectangle;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep_draft, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let floor = rounded_rectangle(0.0, 0.0, 30.0, 20.0, 4.0, 0.0)?;
    Ok(extrude_profile_brep_draft(&floor, p(0.0, 0.0, 1.0), 10.0, 5f64.to_radians(), None)?)
}
