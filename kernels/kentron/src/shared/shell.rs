//! What the shell implementations share: a solid hollowed with `offset::thick_solid`, its openings
//! picked by a point on each, and the solids the cases shell.
//!
//! `thick_solid` keeps the faces it is given (the openings) where they are, moves every other face
//! by the thickness (inward for a negative one, so the solid's faces stay the outside), and closes
//! the wall at each opening by a rim on the opening's surface. Given no opening, it hollows the
//! solid: the moved faces bound a closed cavity, a shell of their own. Joined by arcs
//! (`Join::Arc`), moved faces that part at an edge are joined by a round about it (the wall is the
//! points within the thickness); joined by intersection, they are extended until they meet.
use crate::bench::{face_through, frame, p, v};
use crate::Refusal;
use kentron::math::{Direction, Frame, Point};
use kentron::offset::{self, Join};
use kentron::primitives::{Cuboid, Cylinder, Sphere};
use std::f64::consts::FRAC_PI_2;
use kentron::sketch::{self, Outcome, Step};
use kentron::topology::{Model, Shape};
use kentron::{boolean, sweep};

/// `solid` shelled inward to a wall `thickness` thick, open at the faces through `openings`.
pub fn inward(m: &mut Model, solid: Shape, openings: &[Point], thickness: f64, join: Join) -> Result<Shape, Refusal> {
    let faces = openings.iter().map(|&at| face_through(m, solid, at)).collect::<Result<Vec<_>, _>>()?;
    Ok(offset::thick_solid(m, solid, &faces, -thickness, join)?)
}

/// `solid` shelled outward to a wall `thickness` thick, open at the faces through `openings`.
pub fn outward(m: &mut Model, solid: Shape, openings: &[Point], thickness: f64, join: Join) -> Result<Shape, Refusal> {
    let faces = openings.iter().map(|&at| face_through(m, solid, at)).collect::<Result<Vec<_>, _>>()?;
    Ok(offset::thick_solid(m, solid, &faces, thickness, join)?)
}

/// `solid` hollowed to a wall `thickness` thick with no opening: its faces the outside, a closed
/// cavity inside.
pub fn hollowed(m: &mut Model, solid: Shape, thickness: f64) -> Result<Shape, Refusal> {
    Ok(offset::thick_solid(m, solid, &[], -thickness, Join::Arc)?)
}

/// The plate whose outline in the xy plane is the polygon through `outline`, extruded `height`
/// along +z.
pub fn plate(m: &mut Model, outline: &[(f64, f64)], height: f64) -> Result<Shape, Refusal> {
    let (&(x0, y0), rest) = outline.split_first().ok_or("an empty outline")?;
    let mut steps = vec![Step::Start(x0, y0)];
    steps.extend(rest.iter().map(|&(x, y)| Step::To(x, y)));
    let face = sketch::profile(m, Frame::WORLD, &steps, Outcome::Face)?;
    Ok(sweep::extrude(m, face, v(0.0, 0.0, height))?.shape)
}

/// The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along z
/// about the line through (x, y): the drill a cylinder running 1 past both faces.
pub fn drilled_block(m: &mut Model, x: f64, y: f64) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 20.0, 10.0)).build(m)?;
    let drill = Cylinder::new(3.0, 12.0).with_frame(frame(p(x, y, -1.0), Direction::Z)).build(m)?;
    Ok(boolean::difference(m, block, drill)?)
}

/// A capsule's end: the cylinder of radius 5 on the z axis from z = 0 to z = 10, and on it the
/// half ball of radius 5 centred on (0, 0, 10), joined into one solid.
pub fn capsule_end(m: &mut Model) -> Result<Shape, Refusal> {
    let side = Cylinder::new(5.0, 10.0).build(m)?;
    let dome = Sphere::new(5.0).with_frame(frame(p(0.0, 0.0, 10.0), Direction::Z)).with_latitudes(0.0, FRAC_PI_2).build(m)?;
    Ok(boolean::union(m, side, dome)?)
}

/// The part of the ball of radius 5 centred on the origin below the plane z = `height`: the
/// sphere primitive's zone from its south pole up to the latitude of that plane, closed by a disc.
pub fn ball_below(m: &mut Model, height: f64) -> Result<Shape, Refusal> {
    let top = (height / 5.0).asin();
    Ok(Sphere::new(5.0).with_latitudes(-FRAC_PI_2, top).build(m)?)
}

/// The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2.
pub fn rounded_plate(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 14.0, 10.0)).build(m)?;
    let corners = [(0.0, 0.0), (20.0, 0.0), (20.0, 14.0), (0.0, 14.0)];
    let edges = corners
        .iter()
        .map(|&(x, y)| crate::bench::edge_through(m, block, p(x, y, 5.0)))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(kentron::blend::fillet(m, block, &edges, 2.0)?)
}
