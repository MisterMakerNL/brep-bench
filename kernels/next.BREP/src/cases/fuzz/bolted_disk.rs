//! A disk of radius Rd and thickness T on the z axis, bored rb along its axis and drilled n times
//! rh on the circle of radius Rb from angle φ0, each hole its own cut by a cylinder from z = −1 to
//! T + 1; turned by ψ about the axis (θ, φ).
use crate::solids::boolean::cut;
use crate::solids::fuzz::turned_by;
use crate::solids::p;
use crate::{Params, Refusal};
use brep_kernel::{make_cylinder_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build(p: &Params) -> Result<BrepSolid, Refusal> {
    let (thickness, up) = (p.num("T")?, self::p(0.0, 0.0, 1.0));
    let drill = |x: f64, y: f64, r: f64| make_cylinder_brep(self::p(x, y, -1.0), up, r, thickness + 2.0);
    let mut disk = make_cylinder_brep(self::p(0.0, 0.0, 0.0), up, p.num("Rd")?, thickness)?;
    disk = cut(&disk, &drill(0.0, 0.0, p.num("rb")?)?)?;
    let (n, circle, start) = (p.num("n")?, p.num("Rb")?, p.num("phi0")?.to_radians());
    for i in 0..n as usize {
        let (s, c) = (start + TAU * i as f64 / n).sin_cos();
        disk = cut(&disk, &drill(circle * c, circle * s, p.num("rh")?)?)?;
    }
    Ok(turned_by(&disk, p.num("theta")?, p.num("phi")?, p.num("psi")?)?)
}
