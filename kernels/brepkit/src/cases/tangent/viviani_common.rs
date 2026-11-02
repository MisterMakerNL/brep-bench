//! A ball of radius 10 about the origin and a cylinder of radius 5 on the line x = 5, y = 0,
//! from z = −12 to 12, whose wall touches the sphere from inside at (10, 0, 0) and meets it in
//! Viviani's curve: their common part.
//!
//! Each seam is put where the case asks, away from the curve's crossing: the ball is turned to
//! put its sphere's poles on the y axis and its seam (u = 0, on its frame's +x) through
//! (−10, 0, 0), and the cylinder's seam (brepkit's is on +x) half a turn onto the line x = 0,
//! y = 0, the side of its wall away from (10, 0, 0). (brepkit's ball also has an equator, its
//! two hemispheres' shared edge, which runs through (10, 0, 0) wherever its poles are put on the
//! y axis; the case says nothing of it.)
use brepkit_math::mat::Mat4;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;
use std::f64::consts::{FRAC_PI_2, PI};

use crate::solids::contact::{ball, cylinder_z};
use crate::solids::{common, placed};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let sphere = ball(topo, 10.0, (0.0, 0.0, 0.0))?;
    let sphere = placed(
        topo,
        sphere,
        Mat4::rotation_y(PI) * Mat4::rotation_x(FRAC_PI_2),
    )?;
    let bore = cylinder_z(topo, 5.0, (0.0, 0.0), -12.0, 12.0)?;
    let bore = placed(
        topo,
        bore,
        Mat4::translation(5.0, 0.0, 0.0) * Mat4::rotation_z(PI),
    )?;
    Ok(common(topo, sphere, bore)?)
}
