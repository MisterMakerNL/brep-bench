//! A disk of radius Rd and thickness T on z, bored rb along its axis and drilled n times rh on the
//! circle of radius Rb from angle φ0, each hole its own cut; turned by ψ about the axis (θ, φ).
use crate::sketch::turned_by;
use crate::Params;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

pub fn build(p: &Params) -> Result<Brep, String> {
    let (up, thickness) = ([0., 0., 1.], p.num("T")?);
    // a drill along z through (x, y), running past both faces
    let drill = |x: f64, y: f64, r: f64| prism(&disc([x, y, -1.], up, r), 0., thickness + 2.);
    let mut disk = prism(&disc([0., 0., 0.], up, p.num("Rd")?), 0., thickness)?;
    disk = combined(&disk, &drill(0., 0., p.num("rb")?)?, Op::Cut, 0.)?;
    let (n, start) = (p.num("n")?, p.num("phi0")?.to_radians());
    let (s, c) = start.sin_cos();
    let first = drill(p.num("Rb")? * c, p.num("Rb")? * s, p.num("rh")?)?;
    for k in 0..n as usize {
        // each the first turned about z by k n-ths of a turn, each its own cut
        let turn = Rigid::turn([0., 0., 0.], up, TAU * k as f64 / n);
        disk = combined(&disk, &first.moved(&turn), Op::Cut, 0.)?;
    }
    Ok(turned_by(&disk, p.num("theta")?, p.num("phi")?, p.num("psi")?))
}
