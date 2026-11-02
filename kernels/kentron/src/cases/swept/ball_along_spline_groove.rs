//! A block, the box from (−6, −6, −8) to (36, 26, 0), with a groove cut into its top face by a
//! ball of radius 3 whose centre follows the cubic Bézier curve in the top face with control
//! points (0, 0, 0), (10, 0, 0), (20, 20, 0), (30, 20, 0): everything the ball passes through, cut.
//!
//! kentron has no sweep of a solid (`sweep::pipe` refuses one: "a pipe of a Solid"), so the ball's
//! sweep is built as elementary geometry gives it: a point p of the ball, centre c, moving with it
//! has velocity v + ω × (p − c), which runs along the sphere exactly where v·(p − c) = 0, on the
//! great circle square to the path. So what the ball passes through is the tube that circle's
//! disc sweeps, kept square to the curve, with the ball where it starts and where it ends; the
//! curve never turns tighter than the ball, so the tube does not fold.
use crate::bench::p;
use crate::shared::sweep::disc;
use crate::shared::swept::cut_each;
use kentron::geometry::curve::{BezierCurve, Curve};
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::primitives::{Cuboid, Sphere};
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-6.0, -6.0, -8.0), p(36.0, 26.0, 0.0)).build(m)?;
    let bezier = BezierCurve::new(vec![p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(20.0, 20.0, 0.0), p(30.0, 20.0, 0.0)]);
    let path = make::edge_from_curve(m, Curve::Bezier(bezier))?;
    // The great circle's disc where the curve starts, square to it (it leaves the origin along +x),
    // piped along the curve: its rotation-minimizing frame, as the curve has an inflection where
    // the Frenet frame would flip over.
    let section = disc(m, p(0.0, 0.0, 0.0), Direction::X, 3.0)?;
    let tube = sweep::pipe(m, section, path)?.shape;
    let start = Sphere::new(3.0).with_frame(Frame::from_z(p(0.0, 0.0, 0.0), Direction::Z)).build(m)?;
    let end = Sphere::new(3.0).with_frame(Frame::from_z(p(30.0, 20.0, 0.0), Direction::Z)).build(m)?;
    // The tube and each ball cut from the block in turn.
    cut_each(m, block, &[tube, start, end])
}
