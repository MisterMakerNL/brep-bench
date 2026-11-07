//! A cylinder of radius 5 on the z axis from z = −1 to 10, cut by the kernel's split of a solid by a
//! plane twice: keep what lies above z = 0, then what lies below z = x tan 30°, a plane through the
//! y axis rising toward +x.
use crate::solids::p;
use crate::solids::singular::split_toward;
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, -1.0), p(0.0, 0.0, 1.0), 5.0, 11.0)?;
    let above = split_toward(&cylinder, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0))?;
    // keep z ≤ x tan 30°: the side x sin 30° − z cos 30° ≥ 0
    let (s, c) = 30f64.to_radians().sin_cos();
    Ok(split_toward(&above, p(0.0, 0.0, 0.0), p(s, 0.0, -c))?)
}
