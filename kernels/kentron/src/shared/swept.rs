//! What the swept implementations share.
//!
//! kentron has no sweep of a solid: `sweep::pipe` refuses one ("a pipe of a Solid"), and
//! `sweep::extrude` and `sweep::revolve` take vertices, edges, wires, faces and compounds of them.
//! So where the motion allows it, a solid's sweep is built from what elementary geometry says it
//! is (SPEC.md, "A swept solid"), each implementation saying how beside its code.
use kentron::boolean;
use kentron::math::{Point, Vector};
use kentron::topology::{Model, NodeKind, Shape};

/// A compound of `shapes`.
pub fn compound(m: &mut Model, shapes: &[Shape]) -> kentron::Result<Shape> {
    let c = m.new_empty(NodeKind::Compound);
    for &s in shapes {
        m.add(c, s)?;
    }
    Ok(c)
}

/// The solids of `shape`, each once.
pub fn solids(m: &Model, shape: Shape) -> Vec<Shape> {
    let mut found: Vec<Shape> = Vec::new();
    for s in m.explore(shape, NodeKind::Solid).collect::<Vec<_>>() {
        if !found.iter().any(|f| f.node == s.node) {
            found.push(s);
        }
    }
    found
}

/// The pieces of what a torus passes through as it slides along `slide`, square to its axis:
/// the torus (centre `centre`, axis along y, radii `major` and `minor`) where it starts and
/// where it ends, and each of four quarters of its face slid along `slide`.
///
/// A face slid whole is only what it passes through if the slide never runs along it inside
/// it. The torus's normal at the point turned u about its axis (from +x) and v round its tube
/// is cos v·(cos u·x − sin u·z) + sin v·y, square to a slide along z where sin u = 0, its tube's
/// two circles in the plane through its centre square to the slide, and where cos v = 0, the
/// circles of radius `major` on its tube's two sides (y = ±minor). Split there, into the faces on
/// u ∈ [0, π], [π, 2π] and v ∈ [−π/2, π/2], [π/2, 3π/2], each slides without folding over
/// itself. A point the torus covers at some instant that no face ever crosses stays inside it all
/// the way, so is in it at both ends.
pub fn slid_torus(m: &mut Model, centre: Point, major: f64, minor: f64, slide: Vector) -> kentron::Result<Vec<Shape>> {
    use kentron::geometry::surface::{Surface, Torus as TorusSurface};
    use kentron::math::{Direction, Frame, Transform};
    use kentron::modify::{self, TransformOptions};
    use kentron::primitives::Torus;
    use kentron::{make, sweep};
    use std::f64::consts::{FRAC_PI_2, PI};
    let frame = Frame::new(centre, Direction::Y, Direction::X);
    let start = Torus::new(major, minor).with_frame(frame).build(m)?;
    let end = modify::transform(m, start, &Transform::translation(slide), TransformOptions::default())?;
    let surface = Surface::Torus(TorusSurface::new(frame, major, minor));
    let mut quarters = Vec::new();
    for (u1, u2) in [(0.0, PI), (PI, 2.0 * PI)] {
        for (v1, v2) in [(-FRAC_PI_2, FRAC_PI_2), (FRAC_PI_2, 3.0 * FRAC_PI_2)] {
            quarters.push(make::face_on_domain(m, surface.clone(), u1, u2, v1, v2)?);
        }
    }
    let quarters = compound(m, &quarters)?;
    let slid = sweep::extrude(m, quarters, slide)?.shape;
    let mut pieces = vec![start];
    pieces.extend(solids(m, slid));
    pieces.push(end);
    Ok(pieces)
}

/// `body` with each of `pieces` cut from it in turn, as holes are cut one after another: what
/// is left is `body` less their union, and pieces that only touch are never fused to each other.
pub fn cut_each(m: &mut Model, body: Shape, pieces: &[Shape]) -> kentron::Result<Shape> {
    let mut body = body;
    for &piece in pieces {
        body = boolean::difference(m, body, piece)?;
    }
    Ok(body)
}
