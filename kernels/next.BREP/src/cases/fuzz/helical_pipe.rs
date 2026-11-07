//! A pipe of radius r along the helix of radius R about the z axis that rises p a turn, from
//! (R, 0, 0) through n turns, square to the path, its ends flat: the kernel's helical sweep.
//!
//! `sweep_profile_helix` sweeps a right-handed helix starting at angle zero from the axis's own
//! perpendicular, which for the z axis is +y, so the pipe is turned a quarter turn back about z to
//! start on +x; a left-handed pipe is that one mirrored in the plane y = 0, which keeps its start.
use crate::solids::p;
use crate::solids::sweep::{circle, moved, turn_z};
use crate::{Params, Refusal};
use brep_kernel::{mirror_brep, sweep_profile_helix, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build(p: &Params) -> Result<BrepSolid, Refusal> {
    let origin = self::p(0.0, 0.0, 0.0);
    // the section is placed on the helix by the sweep; only its radius matters
    let section = circle(origin, self::p(1.0, 0.0, 0.0), self::p(0.0, 1.0, 0.0), p.num("r")?)?;
    let coil = sweep_profile_helix(&section, origin, self::p(0.0, 0.0, 1.0), p.num("R")?,
        p.num("p")?, p.num("n")?, None)?;
    let right = moved(&coil, turn_z(-FRAC_PI_2, origin)?)?;
    match p.text("hand")? {
        "right" => Ok(right),
        "left" => Ok(mirror_brep(&right, origin, self::p(0.0, 1.0, 0.0))?),
        hand => Err(Refusal::from(format!("no hand `{hand}`"))),
    }
}
