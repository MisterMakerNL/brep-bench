//! The solid the box from (0, 0, 0) to (10, 6, 4) fills as it slides, without turning, along
//! (8, 5, 3) to the box from (8, 5, 3) to (18, 11, 7).
//!
//! kentron has no sweep of a solid (`sweep::extrude` takes faces, wires, edges, vertices and
//! compounds of them), so the sweep is built as elementary geometry gives it: the box where it
//! starts, the box where it ends, and each of its faces slid along the vector. A point the box
//! covers at some instant that no face ever crosses stays inside the box all the way, so is in it
//! at both ends; and a flat face slid out of its plane never folds over itself, so each face's
//! prism is exactly what that face passes through.
use crate::bench::{p, v};
use crate::shared::swept::{compound, solids};
use kentron::boolean;
use kentron::modify::{self, TransformOptions};
use kentron::math::Transform;
use kentron::primitives::Cuboid;
use kentron::sweep;
use kentron::topology::{Model, NodeKind, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let slide = v(8.0, 5.0, 3.0);
    let start = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 6.0, 4.0)).build(m)?;
    let end = modify::transform(m, start, &Transform::translation(slide), TransformOptions::default())?;
    // The box's six faces, each slid along the vector to the prism it passes through.
    let faces: Vec<Shape> = m.explore(start, NodeKind::Face).collect();
    let faces = compound(m, &faces)?;
    let prisms = sweep::extrude(m, faces, slide)?.shape;
    let mut body = start;
    for piece in solids(m, prisms).into_iter().chain([end]) {
        body = boolean::union(m, body, piece)?;
    }
    Ok(body)
}
