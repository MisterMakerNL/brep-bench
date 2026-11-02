//! A square column tapering as it twists, z from 0 to 40: its section at height z the square of
//! side 30 − z/2 centred on the z axis, turned right-handed by 4.5° per unit of height (half a
//! turn in all), its corners on four conical spirals; each wall swept by a side of the square,
//! a straight horizontal segment between neighbouring spirals. The ends are flat.
//!
//! A sweep up the axis through two sections, the square of side 30 at the foot and the square
//! of side 10 at the top (turned the half turn, so that its corners match the foot's), blended
//! linearly between them, the conical spiral through the corner (15, 15, 0) its guide: at each
//! height the square turns so that its corner keeps pointing at where the plane at that height
//! meets the spiral.
use crate::bench::p;
use crate::shared::loft::polygon;
use kentron::helix::{self, HelixOptions, HelixPart};
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::sweep::{self, Guide, PipeOptions};
use kentron::topology::{Model, Shape};
use std::f64::consts::SQRT_2;

const HEIGHT: f64 = 40.0;
const HALF_START: f64 = 15.0; // the square's half-side at the foot
const HALF_END: f64 = 5.0; // and at the top
const TURNS: f64 = 0.5; // how far it turns, right-handed about z, from foot to top

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (a, b) = (HALF_START, HALF_END);
    let foot = polygon(m, &[p(a, a, 0.0), p(-a, a, 0.0), p(-a, -a, 0.0), p(a, -a, 0.0)])?;
    // the foot's corners turned half a turn: (a, a) is at (−b, −b) at the top
    let top = polygon(m, &[p(-b, -b, HEIGHT), p(b, -b, HEIGHT), p(b, b, HEIGHT), p(-b, b, HEIGHT)])?;
    let axis = make::edge_between(m, p(0.0, 0.0, 0.0), p(0.0, 0.0, HEIGHT))?;
    // the conical spiral through the corner (a, a, 0): its distance from the axis shrinking
    // linearly from a√2 to b√2 as it turns half a turn, right-handed
    let start = Frame::new(p(0.0, 0.0, 0.0), Direction::Z, Direction::new(1.0, 1.0, 0.0));
    let part = HelixPart { height: HEIGHT, pitch: HEIGHT / TURNS, end_diameter: 2.0 * b * SQRT_2 };
    let options = HelixOptions { tolerance: 1e-7, ..HelixOptions::default() };
    let guide = helix::helix(m, &start, 2.0 * a * SQRT_2, &[part], &options)?.wire;
    let guide = Guide::Wire { wire: guide, curvilinear: false };
    let sections = [(foot, PipeOptions::default()), (top, PipeOptions::default())];
    Ok(sweep::pipe_through_with(m, &sections, axis, true, Some(guide))?.shape)
}
