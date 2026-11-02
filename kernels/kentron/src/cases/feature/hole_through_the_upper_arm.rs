//! A C-shaped bracket with a hole drilled through its upper arm and no further.
//!
//! The bracket: the polygon through (x, z) = (0, 0), (40, 0), (40, 6), (6, 6), (6, 24), (40, 24),
//! (40, 30), (0, 30) in the plane y = −10, extruded along +y to y = 10: a lower arm from z = 0 to
//! 6, an upper arm from z = 24 to 30, joined by a back from x = 0 to 6.
//!
//! The hole: radius 4, on the vertical axis through (28, 0), drilled down from the upper arm's top
//! through the first wall it meets, ending where it comes out into the gap at z = 24. kentron has
//! no hole that stops at the next wall by itself, so the hole is its form feature: a pocket of the
//! disc on the arm's top, swept down until the arm's underside, the face the drill comes out of.
use crate::bench::{face_through, frame, p, v};
use crate::Refusal;
use kentron::feature::{self, FeatureExtent, FeatureMode, FeatureSweep};
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    use Step::{Start, To};
    // The section in the plane y = −10, u along x and v along z.
    let plane = Frame::new(p(0.0, -10.0, 0.0), Direction::NEG_Y, Direction::X);
    let outline = [
        Start(0.0, 0.0),
        To(40.0, 0.0),
        To(40.0, 6.0),
        To(6.0, 6.0),
        To(6.0, 24.0),
        To(40.0, 24.0),
        To(40.0, 30.0),
        To(0.0, 30.0),
    ];
    let section = sketch::profile(m, plane, &outline, Outcome::Face)?;
    let bracket = sweep::extrude(m, section, v(0.0, 20.0, 0.0))?.shape;

    let circle = make::edge_from_curve(m, Curve::Circle(Circle::new(frame(p(28.0, 0.0, 30.0), Direction::Z), 4.0)))?;
    let disc = make::wire(m, &[circle])?;
    let underside = face_through(m, bracket, p(28.0, 0.0, 24.0))?;
    Ok(feature::feature(
        m,
        bracket,
        disc,
        FeatureSweep::Prism(Direction::NEG_Z),
        FeatureMode::Remove,
        FeatureExtent::Until(underside),
    )?)
}
