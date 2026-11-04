//! The half with y ≥ 0 of the solid torus about the z axis, its tube of radius 2 about a middle
//! circle of radius 6, shelled inward to a wall 0.5 thick, open at both flat ends: a curved pipe,
//! the half torus less the half torus of tube radius 1.5. The half torus is the tube's disc in the
//! plane y = 0 turned a half turn about z, from +x towards +y.
use crate::bench::{circle, faces_through_each, order};
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{shell, Body};
use std::f64::consts::PI;

pub fn build() -> Result<Body, Refusal> {
    let disc = [circle([6.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0], 2.0)];
    let half = revolve(&disc, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], PI, true)?;
    let ends = faces_through_each(&half, &[[6.0, 0.0, 0.0], [-6.0, 0.0, 0.0]])?;
    shell(&half, &order(&half)?, &ends, -0.5)?;
    Ok(half)
}
