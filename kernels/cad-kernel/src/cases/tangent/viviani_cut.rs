//! A ball of radius 10 at the origin and a cylinder of radius 5 on the line x = 5, y = 0 from
//! z = −12 to 12, tangent inside the ball at (10, 0, 0): the ball less the cylinder, bounded where they meet by
//! Viviani's curve, which crosses itself at that point.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{ball_section, prism, revolve, Profile, ProfileEdge};
use cad_kernel::geom::Frame;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const RADIUS: f64 = 10.;
const CYLINDER_RADIUS: f64 = RADIUS / 2.;
const HALF_HEIGHT: f64 = 12.;
/// Where each seam goes, away from the curve's crossing at (10, 0, 0), as the case asks.
const AWAY: [f64; 3] = [-1., 0., 0.];

pub fn build() -> Result<Brep, String> {
    // the ball: a half disc in the xy plane on the side of −x, shut along the y axis and turned
    // about it, so its poles are on the y axis and its seam runs through (−10, 0, 0)
    let half = Frame::new([0., 0., 0.], [0., 0., -1.], AWAY);
    let ball = revolve(&ball_section(&half, RADIUS), half.o, half.y, TAU)?;
    // the cylinder: its circle starting from the side towards −x, so its seam is on the z axis
    let foot = [CYLINDER_RADIUS, 0., -HALF_HEIGHT];
    let circle = ProfileEdge::Arc { frame: Frame::new(foot, [0., 0., 1.], AWAY), r: CYLINDER_RADIUS, span: None };
    let disc = Profile { origin: foot, normal: [0., 0., 1.], loops: vec![vec![circle]], names: Vec::new() };
    let cylinder = prism(&disc, 0., 2. * HALF_HEIGHT)?;
    Ok(combined(&ball, &cylinder, Op::Cut, 0.)?)
}
