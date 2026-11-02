//! A solid cone on the disc of radius 5 about the z axis in z = 0, apex (0, 0, 10), whose side is
//! one NURBS surface with a pole at the apex: rational quadratic round the axis (the base circle
//! as nine weighted poles on the knots 0, ¼, ½, ¾, 1) and linear up it (the second row nine
//! copies of the apex, same weights). That face and the base's disc are sewn into the shell
//! that bounds the cone; no cone is used.
//!
//! kentron seams a face only on a periodic surface, so the circle round the axis is taken in
//! its periodic form, the same curve (see `circle_nurbs_periodic`): the net's first eight
//! columns, its ninth the first again.
use crate::bench::p;
use crate::shared::geometry::{circle_nurbs_periodic, flat, solid_of};
use crate::Refusal;
use kentron::geometry::curve::{Circle, Curve};
use kentron::geometry::surface::{BSplineSurface, KnotVector, Surface};
use kentron::make;
use kentron::math::Frame;
use kentron::topology::{Model, Shape};

const R: f64 = 5.0;
const HEIGHT: f64 = 10.0;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let (circle, w, knots, mults) = circle_nurbs_periodic();
    let poles = circle.iter().map(|&(x, y)| vec![p(R * x, R * y, 0.0), p(0.0, 0.0, HEIGHT)]).collect();
    let weights = w.iter().map(|&wi| vec![wi, wi]).collect();
    let up = KnotVector::new(vec![0.0, 1.0], vec![2, 2], 1);
    let net = BSplineSurface::try_new(poles, KnotVector::periodic(knots, mults, 2), up)?.with_weights(weights);
    let side = make::face_on_surface(m, Surface::BSpline(net))?;
    let rim = make::edge_from_curve(m, Curve::Circle(Circle::new(Frame::WORLD, R)))?;
    let base = flat(m, &[rim])?;
    solid_of(m, &[side, base])
}
