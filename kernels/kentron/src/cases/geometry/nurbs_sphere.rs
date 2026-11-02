//! A ball of radius 5 centred on the origin, its whole surface the standard biquadratic NURBS
//! sphere: a 9 × 5 net, the unit circle (nine weighted poles on the knots 0, ¼, ½, ¾, 1) round
//! the axis times the semicircle (ρ, z) = (0, −5), (5, −5), (5, 0), (5, 5), (0, 5) on the knots
//! 0, ½, 1 along the meridian, each pole's weight the product. Its rows j = 0 and j = 4 collapse
//! to the poles. The one face over the whole net, closed on its seam (the meridian in y = 0,
//! x ≥ 0) with degenerate edges at its poles, is the closed shell bounding the ball; no sphere
//! is used.
//!
//! kentron seams a face only on a periodic surface, so the circle round the axis is taken in
//! its periodic form, the same curve (see `circle_nurbs_periodic`): the net's first eight
//! columns, its ninth the first again.
use crate::bench::p;
use crate::shared::geometry::{circle_nurbs_periodic, solid_of};
use crate::Refusal;
use kentron::geometry::surface::{BSplineSurface, KnotVector, Surface};
use kentron::make;
use kentron::topology::{Model, Shape};

const R: f64 = 5.0;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let (circle, w, knots, mults) = circle_nurbs_periodic();
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let meridian = [(0.0, -R), (R, -R), (R, 0.0), (R, R), (0.0, R)];
    let meridian_w = [1.0, h, 1.0, h, 1.0];
    let poles = circle.iter().map(|&(cx, cy)| meridian.iter().map(|&(rho, z)| p(rho * cx, rho * cy, z)).collect()).collect();
    let weights = w.iter().map(|&wi| meridian_w.iter().map(|&wj| wi * wj).collect()).collect();
    let net = BSplineSurface::try_new(
        poles,
        KnotVector::periodic(knots, mults, 2),
        KnotVector::new(vec![0.0, 0.5, 1.0], vec![3, 2, 3], 2),
    )?
    .with_weights(weights);
    let face = make::face_on_surface(m, Surface::BSpline(net))?;
    solid_of(m, &[face])
}
