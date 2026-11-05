//! A hub, a stop and a rib turned around the hub until it meets the stop, all joined. Angles about
//! the z axis are from +x toward +y.
//! - the hub: the cylinder of radius 10 on the z axis from z = 0 to z = 10;
//! - the stop: everything within 25 of the axis between the half-planes at 120° and 135°, from
//!   z = 0 to z = 10, the rectangle 0 ≤ r ≤ 25, 0 ≤ z ≤ 10 revolved from 120° through 15°;
//! - the rib: the rectangle 5 ≤ x ≤ 20, 0 ≤ z ≤ 4 in the plane y = 0 revolved about the z axis from
//!   0° to the stop's face at 120°.
use crate::solids::boolean::union;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, revolve_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, z) = (p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let hub = make_cylinder_brep(o, z, 10.0, 10.0)?;
    let (c, s) = (120f64.to_radians().cos(), 120f64.to_radians().sin());
    let at120 = |r: f64, h: f64| p(r * c, r * s, h);
    let stop_section = polygon(&[at120(0.0, 0.0), at120(25.0, 0.0), at120(25.0, 10.0), at120(0.0, 10.0)])?;
    let stop = revolve_profile_brep(&stop_section, o, z, 15f64.to_radians())?;
    let rib_section = polygon(&[p(5.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 0.0, 4.0), p(5.0, 0.0, 4.0)])?;
    let rib = revolve_profile_brep(&rib_section, o, z, 120f64.to_radians())?;
    let hub_and_stop = union(&hub, &stop)?;
    Ok(union(&hub_and_stop, &rib)?)
}
