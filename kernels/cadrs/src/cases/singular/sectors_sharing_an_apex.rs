//! Two spherical sectors of a ball of radius 10, each the part of the ball within 45° of its
//! cone's axis, both with their apex at the origin: the first about the z axis toward +z, the
//! second the first turned 60° about the y axis. The solid common to both. Their caps lie on the
//! same sphere, and the two cones' sides meet in two straight lines from the apex.
//!
//! The first sector is its section beside the z axis, sketched on the Front (xz) plane, revolved
//! a whole turn about z (cadrs has no sphere or cone primitive): from the apex along the cone's
//! ruling to (10 sin 45°, 10 cos 45°), along the sphere's arc to the pole (0, 10), and down the
//! axis. The second is a copy turned 60° about y; the two are intersected.
use crate::bench::{arc, axis, chain, common, line, p2, p3, revolve, sketch, turned, v3, xz};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let rim = p2(10.0 * 45f64.to_radians().sin(), 10.0 * 45f64.to_radians().cos());
    let section = chain(vec![
        line(p2(0.0, 0.0), rim),
        arc(p2(0.0, 0.0), 10.0, 45.0, 45.0),
        line(p2(0.0, 10.0), p2(0.0, 0.0)),
    ]);
    let s = sketch(xz(0.0), vec![section]);
    let origin = p3(0.0, 0.0, 0.0);
    let first = revolve(k, &s, axis(origin, v3(0.0, 0.0, 1.0)), None)?;
    let second = turned(k, first, axis(origin, v3(0.0, 1.0, 0.0)), 60.0)?;
    common(k, first, second)
}
