//! A hub, the cylinder of radius 10 on the z axis from z = 0 to z = 10; a stop, everything within
//! 25 of the z axis between the half-planes at 120° and 135° from z = 0 to z = 10; and a rib, the
//! rectangle 5 ≤ x ≤ 20, 0 ≤ z ≤ 4 in the plane y = 0 turned about the z axis from 0° towards +y
//! until it meets the stop's face at 120°; all three joined.
//!
//! solidcore-ops has no revolution up to a face, so the rib is turned through the 120° to the
//! stop's face.
use crate::bench::{fuse_all, polygon};
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let z = [0.0, 0.0, 1.0];
    let hub = make_cylinder([0.0, 0.0, 0.0], 10.0, 10.0)?;
    // the stop: its section in the half-plane at 120°, turned 15° on to 135°
    let (c, s) = (120f64.to_radians().cos(), 120f64.to_radians().sin());
    let stop_section =
        polygon(&[[0.0, 0.0, 0.0], [25.0 * c, 25.0 * s, 0.0], [25.0 * c, 25.0 * s, 10.0], [0.0, 0.0, 10.0]]);
    let stop = revolve(&stop_section, [0.0; 3], z, 15f64.to_radians(), true)?;
    let rib_section =
        polygon(&[[5.0, 0.0, 0.0], [20.0, 0.0, 0.0], [20.0, 0.0, 4.0], [5.0, 0.0, 4.0]]);
    let rib = revolve(&rib_section, [0.0; 3], z, 120f64.to_radians(), true)?;
    fuse_all([hub, stop, rib])
}
