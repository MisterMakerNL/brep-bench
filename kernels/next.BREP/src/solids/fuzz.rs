//! Helpers shared by the families (`fuzz/`): an axis named by its two angles, and a solid turned
//! about such an axis through the origin, as a family's description places its solid.
#![allow(unused_imports)]
use super::*;
use crate::solids::singular::turned;
use brep_kernel::{BrepSolid, Vec3};

/// The unit direction (sin θ cos φ, sin θ sin φ, cos θ), θ and φ in degrees.
pub fn direction(theta: f64, phi: f64) -> Vec3 {
    let (t, f) = (theta.to_radians(), phi.to_radians());
    p(t.sin() * f.cos(), t.sin() * f.sin(), t.cos())
}

/// `solid` turned by `psi` degrees, right-handed, about the axis through the origin with direction
/// (`theta`, `phi`).
pub fn turned_by(solid: &BrepSolid, theta: f64, phi: f64, psi: f64) -> Result<BrepSolid, String> {
    turned(solid, p(0.0, 0.0, 0.0), direction(theta, phi), psi.to_radians(), p(0.0, 0.0, 0.0))
}
