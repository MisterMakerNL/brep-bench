//! The solid paraboloid x² + y² ≤ 10z, z ≤ 5 (the parabola x² = 10z, focal length 2.5, turned a
//! whole turn about the z axis and closed by the disc at z = 5), united with the ball of radius 5
//! about (0, 0, 5), its poles on the z axis: the ball's bottom pole sits in the cup's vertex, where
//! the two curve alike.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_line, make_parabola, make_sphere_brep, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    // P(t) = (2·2.5·t, 2.5·t²) in the frame (axis +z, latus +x): t = √2 reaches z = 5
    let rim = (50.0f64).sqrt();
    let section = [
        make_parabola(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), p(1.0, 0.0, 0.0), 2.5, 0.0, 2f64.sqrt())?,
        make_line(p(rim, 0.0, 5.0), p(0.0, 0.0, 5.0))?,
        make_line(p(0.0, 0.0, 5.0), p(0.0, 0.0, 0.0))?,
    ];
    let cup = revolve_profile_brep(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?;
    let ball = make_sphere_brep(p(0.0, 0.0, 5.0), 5.0, p(0.0, 0.0, 1.0))?;
    Ok(union(&cup, &ball)?)
}
