//! The block from (−15, −15, 0) to (15, 15, 20), bored through along the z axis by a hole of
//! radius 6, the hole's wall drafted 3° for the pull direction +z about the neutral plane z = 0,
//! the floor: the kernel's draft makes the wall the cone through the hole's circle at z = 0,
//! widening as it rises, and re-trims the top to it.
use crate::solids::boolean::cut;
use crate::solids::draft::faces_at;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{draft_faces, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-15.0, -15.0, 0.0), 30.0, 30.0, 20.0)?;
    // The drill runs past both faces.
    let drill = make_cylinder_brep(p(0.0, 0.0, -5.0), p(0.0, 0.0, 1.0), 6.0, 30.0)?;
    let solid = cut(&block, &drill)?;
    let wall = faces_at(&solid, &[p(6.0, 0.0, 10.0)])?;
    Ok(draft_faces(&solid, &wall, p(0.0, 0.0, 1.0), p(0.0, 0.0, 0.0), 3f64.to_radians())?)
}
