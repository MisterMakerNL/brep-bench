//! A closed hollow dome with a post inside it. The dome: a plate, the disk of radius 20 about
//! the z axis from z = −2 to z = 0, and on it a hemispherical shell between 18 and 20 from the
//! origin, closing the cavity of the half ball of radius 18 above the plate. The post: the disk
//! of radius 3 on the cavity's floor extruded up until it meets the inside of the dome, the
//! sphere of radius 18, and joined to the dome.
//!
//! The dome is its section beside the z axis (the plate's rectangle, the shell's quarter annulus
//! on it) revolved a whole turn: cadrs has no sphere primitive. The post is the disk sketched on
//! the floor's plane z = 0 and extruded up "up to face", the dome's inner sphere, so its top
//! takes the sphere's shape; the two are united.
use crate::bench::{
    arc, axis, chain, face_at, line, p2, p3, revolve, round, sketch, through_axis, union, v3, xy,
};
use crate::shared::feature::extrude_to;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, ExtrudeEnd};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // the section in (radius, z)
    let section = chain(vec![
        line(p2(0.0, -2.0), p2(20.0, -2.0)),
        line(p2(20.0, -2.0), p2(20.0, 0.0)),
        arc(p2(0.0, 0.0), 20.0, 0.0, 90.0),
        line(p2(0.0, 20.0), p2(0.0, 18.0)),
        arc(p2(0.0, 0.0), 18.0, 90.0, -90.0),
        line(p2(18.0, 0.0), p2(0.0, 0.0)),
        line(p2(0.0, 0.0), p2(0.0, -2.0)),
    ]);
    let z = v3(0.0, 0.0, 1.0);
    let origin = p3(0.0, 0.0, 0.0);
    let dome = revolve(k, &sketch(through_axis(origin, z), vec![section]), axis(origin, z), None)?;
    let inside = face_at(k, dome, p3(0.0, 0.0, 18.0))?;
    let disk = sketch(xy(0.0), vec![round((0.0, 0.0), 3.0)]);
    let end = ExtrudeEnd::UpToFace { body: dome, face: inside, offset: 0.0 };
    let post = extrude_to(k, &disk, z, end, &[])?;
    union(k, dome, post)
}
