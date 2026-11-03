//! A hub with a stop, and a rib turned round the hub until it meets the stop. The hub: a cylinder
//! of radius 10 on the z axis from z = 0 to 10. The stop: everything within 25 of the z axis
//! between the half-planes at 120° and 135° from +x, from z = 0 to 10, joined to the hub. The rib:
//! the rectangle 5 ≤ x ≤ 20, 0 ≤ z ≤ 4 of the plane y = 0, turned about z from +x towards +y until
//! it meets the stop, and joined to them.
//!
//! cad-kernel's revolve turns through a given angle and has no revolution up to a face. So the rib
//! is turned through 120°, the angle of the stop's face it is to meet, where its end lies flat
//! against that face.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, revolve};
use cad_kernel::topo::Brep;

const HUB_RADIUS: f64 = 10.;
const HEIGHT: f64 = 10.;
const STOP_REACH: f64 = 25.;
const STOP_FROM: f64 = 120.;
const STOP_TO: f64 = 135.;
const RIB_FROM: f64 = 5.;
const RIB_TO: f64 = 20.;
const RIB_HEIGHT: f64 = 4.;

/// The rectangle from `r0` to `r1` out from the z axis and from z = 0 to `h`, in the half-plane at
/// `angle` from +x.
fn upright(angle: f64, r0: f64, r1: f64, h: f64) -> cad_kernel::build::Profile {
    let (s, c) = angle.to_radians().sin_cos();
    let at = |r: f64, z: f64| [r * c, r * s, z];
    polygon(&[at(r0, 0.), at(r1, 0.), at(r1, h), at(r0, h)])
}

pub fn build() -> Result<Brep, String> {
    let z = [0., 0., 1.];
    let hub = prism(&disc([0., 0., 0.], z, HUB_RADIUS), 0., HEIGHT)?;
    // the stop: its face at 120° turned on to 135°
    let stop = revolve(&upright(STOP_FROM, 0., STOP_REACH, HEIGHT), [0., 0., 0.], z, (STOP_TO - STOP_FROM).to_radians())?;
    let base = combined(&hub, &stop, Op::Union, 0.)?;
    let rib = revolve(&upright(0., RIB_FROM, RIB_TO, RIB_HEIGHT), [0., 0., 0.], z, STOP_FROM.to_radians())?;
    Ok(combined(&base, &rib, Op::Union, 0.)?)
}
