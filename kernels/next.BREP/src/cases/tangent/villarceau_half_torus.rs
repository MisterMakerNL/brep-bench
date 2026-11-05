//! The solid torus about the z axis (middle circle of radius 5, tube of radius 3), cut by the
//! bitangent plane 3x − 4z = 0, keeping what lies on and below it, z ≤ 3x/4. The kernel's torus
//! about z turns its tube's section from +y, the seam in the plane x = 0 on the side y > 0.
use crate::solids::p;
use crate::solids::singular::keep_toward;
use crate::Refusal;
use brep_kernel::{make_torus_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let torus = make_torus_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 3.0)?;
    // the side 3x − 4z ≥ 0 is z ≤ 3x/4
    keep_toward(&torus, p(0.0, 0.0, 0.0), p(3.0, 0.0, -4.0), 20.0)
}
